from http import HTTPStatus
from time import time
from typing import Iterable, List, Union
from json import loads, dumps
from ..interfaces import User as IUser, Chat as IChat, ChatMember as IChatMember, ChatMessage as IChatMessage, ChatMemberPermissions, ApiError, Event as IEvent
from .database import DbUser, DbChat, DbChatMember, DbChatMessage, DbEvent
from ..events import MemberAdded, MemberKicked, NewMessage
from bcrypt import hashpw, checkpw, gensalt
import peewee

UTF_8 = 'utf-8'


class User(IUser):
    def __init__(self, id: int, username: str, password: str | None = None, email: str | None = None, is_bot: bool = False, creator_id: int | None = None, last_login_timestamp: float = 0) -> None:
        self._id = id
        self._username = username
        self._password = password
        self._email = email
        self._is_bot = is_bot
        self._creator_id = creator_id
        self._last_login_timestamp = last_login_timestamp

    @classmethod
    @ApiError.wrap_exception(peewee.IntegrityError, HTTPStatus.CONFLICT, "User already exists")
    def new(cls, username: str, password: str, email: str) -> "User":
        hashed_password = hashpw(
            bytes(password, UTF_8), gensalt()).decode(UTF_8)
        new_user = DbUser.create(
            username=username,
            email=email,
            password=hashed_password
        )

        return User.from_db_model(new_user)

    @classmethod
    @ApiError.wrap_exception(peewee.IntegrityError, HTTPStatus.CONFLICT, "Bot already exists")
    def new_bot(cls, username: str, creator: "User") -> "User":
        new_user = DbUser.create(
            username=username,
            creator_id=creator.id(),
            is_bot=True
        )

        return User.from_db_model(new_user)

    def id(self) -> int:
        return self._id

    def username(self) -> str:
        return self._username

    def email(self) -> str:
        return self._email

    def set_email(self, value: str):
        DbUser.update(email=value).where(DbUser.id == self.id()).execute()
        self._email = value

    def last_login_timestamp(self) -> float:
        return int(self._last_login_timestamp)

    def update_login_timestamp(self):
        new_time = int(time())
        DbUser.update(last_login_timestamp=new_time).where(
            DbUser.id == self.id()).execute()
        self._last_login_timestamp = new_time

    def get_unread_events(self) -> Iterable[IEvent]:
        events = DbEvent.filter((DbEvent.is_read == False)
                                & (DbEvent.user_id == self.id()))
        return [Event.from_db_model(el) for el in events]

    def compare_password(self, raw_password: str) -> bool:
        return checkpw(bytes(raw_password, UTF_8), bytes(self._password, UTF_8))

    def set_password(self, value: str):
        hashed_password = hashpw(bytes(value, UTF_8), gensalt()).decode(UTF_8)
        DbUser.update(password=hashed_password).where(
            DbUser.id == self.id()).execute()
        self._password = hashed_password

    def is_bot(self) -> bool:
        return self._is_bot

    def owned_bots(self) -> Iterable["User"]:
        bots = DbUser.filter((DbUser.creator_id == self.id())
                             & (DbUser.is_bot == True))
        return [User.from_db_model(el) for el in bots]

    def creator(self) -> Union["User", None]:
        if self._creator_id is None:
            return None

        creator_user = DbUser.get(id=self._creator_id)
        return self.from_db_model(creator_user)

    def chats(self) -> Iterable["Chat"]:
        chat_members = [ChatMember.from_db_model(
            el) for el in DbChatMember.filter((DbChatMember.user_id == self.id()))]
        chats = [el.chat() for el in chat_members]
        return chats

    def delete(self):
        DbUser.delete().where(DbUser.id == self.id()).execute()

    @classmethod
    def search_users(cls, username: str, offset: int = 0, limit: int = 10) -> Iterable["User"]:
        users = DbUser.select().where(
            (DbUser.username % f'%{username}%') &
            ~(DbUser.is_bot)
        ).offset(offset).limit(limit)

        return [User.from_db_model(el) for el in users]

    @classmethod
    def search_bots(cls, username: str, offset: int = 0, limit: int = 10) -> Iterable["User"]:
        users = DbUser.select().where(
            (DbUser.username % f'%{username}%') &
            (DbUser.is_bot)
        ).offset(offset).limit(limit)

        return [User.from_db_model(el) for el in users]

    @classmethod
    @ApiError.wrap_exception(peewee.DoesNotExist, HTTPStatus.NOT_FOUND, "User does not exists")
    def get_by_id(cls, id: int) -> "User":
        user = DbUser.get(id=id)
        return cls.from_db_model(user)

    @classmethod
    @ApiError.wrap_exception(peewee.DoesNotExist, HTTPStatus.NOT_FOUND, "User does not exists")
    def get_by_username(cls, username: str) -> "User":
        user = DbUser.get(username=username)
        return cls.from_db_model(user)

    @staticmethod
    def from_db_model(db_model: DbUser) -> "User":
        return User(
            id=db_model.id,
            username=db_model.username,
            password=db_model.password,
            email=db_model.email,
            is_bot=db_model.is_bot,
            creator_id=db_model.creator_id,
            last_login_timestamp=db_model.last_login_timestamp
        )


class Chat(IChat):
    def __init__(self, id: int, name: str, owner_id: int) -> None:
        self._id = id
        self._name = name
        self._owner_id = owner_id

    def id(self) -> int:
        return self._id

    def name(self) -> str:
        return self._name

    @classmethod
    def new(cls, name: str, owner: User):
        pg_chat = DbChat.create(
            name=name,
            owner_id=owner.id()
        )

        chat = Chat.from_db_model(pg_chat)
        chat_member = chat.add_member(owner)
        chat_member.set_permissions(ChatMemberPermissions(True, True, True))

        return chat

    def send_message(self, sender: User, content: str, files: List[str]) -> "ChatMessage":
        pg_message = DbChatMessage.create(
            sender_id=sender.id(),
            content=content,
            chat_id=self.id(),
            files=ChatMessage.FILES_SPLITTER.join(files)
        )

        message = ChatMessage.from_db_model(pg_message)

        event_payload = NewMessage(message).to_dict()
        self.send_event_to_members(event_payload)

        return message

    def set_name(self, value: str):
        DbChat.update(name=value).where(DbChat.id == self.id()).execute()
        self._name = value

    def owner(self) -> User:
        return User.get_by_id(self._owner_id)

    def set_owner(self, user: User):
        DbChat.update(owner_id=user.id()).where(
            DbChat.id == self.id()).execute()

    def members(self) -> Iterable["ChatMember"]:
        members = DbChatMember.filter((DbChatMember.chat_id == self.id()))
        return [ChatMember.from_db_model(el) for el in members]

    @ApiError.wrap_exception(peewee.IntegrityError, HTTPStatus.CONFLICT, "User already in chat")
    def add_member(self, user: User) -> "ChatMember":
        pg_member = DbChatMember.create(user_id=user.id(), chat_id=self.id())
        member = ChatMember.from_db_model(pg_member)

        event_payload = MemberAdded(member).to_dict()
        self.send_event_to_members(event_payload)
        return member

    def delete(self):
        DbChat.delete().where(DbChat.id == self.id()).execute()

    def messages(self, offset: int, limit: int) -> Iterable["ChatMessage"]:
        messages = DbChatMessage.select().where(
            (DbChatMessage.chat_id == self.id())
        ).order_by(DbChatMessage.id.desc()).offset(offset).limit(limit)

        return [ChatMessage.from_db_model(el) for el in messages]

    def send_event_to_members(self, event_payload: dict):
        for chat_member in self.members():
            user = chat_member.user()
            Event.new(user, event_payload)

    @classmethod
    @ApiError.wrap_exception(peewee.DoesNotExist, HTTPStatus.NOT_FOUND, "Chat does not exists")
    def get_by_id(cls, id: int) -> "Chat":
        chat = DbChat.get(id=id)
        return cls.from_db_model(chat)

    @staticmethod
    def from_db_model(db_model: DbChat):
        return Chat(
            db_model.id,
            db_model.name,
            db_model.owner_id
        )


class ChatMember(IChatMember):
    def __init__(self, id: int, chat_id: int, user_id: int, permissions: ChatMemberPermissions) -> None:
        self._id = id
        self._chat_id = chat_id
        self._user_id = user_id
        self._permissions = permissions

    def id(self) -> int:
        return self._id

    def chat(self) -> Chat:
        return Chat.get_by_id(self._chat_id)

    def user(self) -> User:
        return User.get_by_id(self._user_id)

    def permissions(self) -> ChatMemberPermissions:
        return self._permissions

    def set_permissions(self, value: ChatMemberPermissions):
        DbChatMember.update(
            can_write=value.can_write,
            can_add_members=value.can_add_members,
            can_kick_members=value.can_kick_members
        ).where(DbChatMember.id == self.id()).execute()

        self._permissions = value

    def delete(self):
        chat = self.chat()
        event_payload = MemberKicked(self.user(), chat).to_dict()
        DbChatMember.delete().where(DbChatMember.id == self.id()).execute()
        chat.send_event_to_members(event_payload)

    @classmethod
    @ApiError.wrap_exception(peewee.DoesNotExist, HTTPStatus.NOT_FOUND, "User is not in chat")
    def get_by_chat_and_user(cls, chat: Chat, user: User):
        member = DbChatMember.get(chat_id=chat.id(), user_id=user.id())
        return ChatMember.from_db_model(member)

    @staticmethod
    def from_db_model(db_model: DbChatMember) -> "ChatMember":
        return ChatMember(
            db_model.id,
            db_model.chat_id,
            db_model.user_id,
            ChatMemberPermissions(
                db_model.can_write,
                db_model.can_add_members,
                db_model.can_kick_members
            )
        )


class ChatMessage(IChatMessage):
    FILES_SPLITTER = ";"

    def __init__(self, id: int, sender_id: int, chat_id: int, content: str, created_timestamp: float, files: List[str]) -> None:
        self._id = id
        self._sender_id = sender_id
        self._chat_id = chat_id
        self._content = content
        self._created_timestamp = created_timestamp
        self._files = files

    def id(self) -> int:
        return self._id

    def chat(self) -> Chat:
        return Chat.get_by_id(self._chat_id)

    def sender(self) -> User:
        return User.get_by_id(self._sender_id)

    def content(self) -> str:
        return self._content

    def files(self) -> Iterable[str]:
        return self._files

    def set_files(self, value: List[str]):
        DbChatMessage.update(files=ChatMessage.FILES_SPLITTER.join(value)).where(
            DbChatMessage.id == self.id()).execute()
        self._files = value

    def set_content(self, value: str):
        DbChatMessage.update(content=value).where(
            DbChatMessage.id == self.id()).execute()
        self._content = value

    def timestamp(self) -> float:
        return self._created_timestamp

    def delete(self):
        DbChatMessage.delete().where(DbChatMessage.id == self.id()).execute()

    @staticmethod
    def from_db_model(db_model: DbChatMessage):
        return ChatMessage(
            db_model.id,
            db_model.sender_id,
            db_model.chat_id,
            db_model.content,
            db_model.created_timestamp,
            db_model.files.split(ChatMessage.FILES_SPLITTER)
        )

    @classmethod
    @ApiError.wrap_exception(peewee.DoesNotExist, HTTPStatus.NOT_FOUND, "Message does not exist")
    def get_by_id(cls, id: int) -> "ChatMessage":
        message = DbChatMessage.get(id=id)
        return cls.from_db_model(message)


class Event(IEvent):
    def __init__(self, id: int, receiver_id: int, is_read: bool, timestamp: float, payload: dict) -> None:
        self._id = id
        self._receiver_id = receiver_id
        self._is_read = is_read
        self._timestamp = timestamp
        self._payload = payload

    @classmethod
    def new(cls, receiver: IUser, payload: dict) -> IEvent:
        str_payload = dumps(payload)

        new_event = DbEvent.create(
            user_id=receiver.id(),
            payload=str_payload
        )

        return Event.from_db_model(new_event)

    def id(self) -> int:
        return self._id

    def receiver(self) -> User:
        user = DbUser.get(id=self._receiver_id)
        return User.from_db_model(user)

    def is_read(self) -> bool:
        return self._is_read

    def create_timestamp(self) -> float:
        return self._timestamp

    def payload(self) -> dict:
        return self._payload

    def mark_as_read(self):
        DbEvent.update(is_read=True).where(DbEvent.id == self.id()).execute()

    @classmethod
    @ApiError.wrap_exception(peewee.DoesNotExist, HTTPStatus.NOT_FOUND, "Event does not exist")
    def get_by_id(cls, id: int) -> "Event":
        event = DbEvent.get(id=id)
        return cls.from_db_model(event)

    @staticmethod
    def from_db_model(db_model: DbEvent):
        return Event(
            db_model.id,
            db_model.user_id,
            db_model.is_read,
            db_model.created_timestamp,
            loads(db_model.payload)
        )
