from http import HTTPStatus
from typing import Iterable, List, Union
from json import loads, dumps
from ..interfaces import User as IUser, Chat as IChat, ChatMember as IChatMember, ChatMessage as IChatMessage, ChatMemberPermissions, ModelError, Event as IEvent
from .database import PgUser, PgChat, PgChatMember, PgChatMessage, PgEvent, objects
from ..events import MemberAdded, MemberKicked, NewMessage
from bcrypt import hashpw, checkpw, gensalt
import peewee

UTF_8 = 'utf-8'

class User(IUser):
    def __init__(self, id: int, username: str, password: str | None = None, email: str | None = None, is_bot: bool = False, creator_id: int | None = None) -> None:
        self._id = id
        self._username = username
        self._password = password
        self._email = email
        self._is_bot = is_bot
        self._creator_id = creator_id
    
    @classmethod
    @ModelError.wrap_async_exception(peewee.IntegrityError, HTTPStatus.CONFLICT, "User already exists")
    async def new(cls, username: str, password: str, email: str) -> "User":
        hashed_password = hashpw(bytes(password, UTF_8), gensalt()).decode(UTF_8)
        new_user = await objects.create(
            PgUser,
            username=username,
            email=email,
            password=hashed_password
        )
        return User.from_db_model(new_user)
    
    @classmethod
    @ModelError.wrap_async_exception(peewee.IntegrityError, HTTPStatus.CONFLICT, "Bot already exists")
    async def new_bot(cls, username: str, creator: "User") -> "User":
        new_user = await objects.create(
            PgUser,
            username=username,
            creator_id=creator.id(),
            is_bot=True,
        )
        return User.from_db_model(new_user)
    
    def id(self) -> int:
        return self._id
    
    def username(self) -> str:
        return self._username
    
    def email(self) -> str:
        return self._email
    
    async def set_email(self, value: str):
        await objects.execute(PgUser.update(email=value).where(PgUser.id == self.id()))
        self._email = value

    async def get_unread_events(self) -> Iterable[IEvent]:
        events = await objects.execute(PgEvent.filter((PgEvent.is_read==False) & (PgEvent.user_id == self.id())))
        return [Event.from_db_model(el) for el in events]

    def compare_password(self, raw_password: str) -> bool:
        return checkpw(bytes(raw_password, UTF_8), bytes(self._password, UTF_8))

    async def set_password(self, value: str):
        hashed_password = hashpw(bytes(value, UTF_8), gensalt()).decode(UTF_8)
        await objects.execute(PgUser.update(password=hashed_password).where(PgUser.id == self.id()))
        self._password = hashed_password

    def is_bot(self) -> bool:
        return self._is_bot
    
    async def owned_bots(self) -> Iterable["User"]:
        bots = await objects.execute(PgUser.filter((PgUser.creator_id == self.id()) & (PgUser.is_bot == True)))
        return [User.from_db_model(el) for el in bots]
    
    async def creator(self) -> Union["User", None]:
        if self._creator_id is None:
            return None
        
        creator_user = await objects.get(PgUser, id=self._creator_id)
        return self.from_db_model(creator_user)
    
    async def chats(self) -> Iterable["Chat"]:
        chat_members = [ChatMember.from_db_model(el) for el in await objects.execute(PgChatMember.filter((PgChatMember.user_id == self.id())))]
        chats = [await el.chat() for el in chat_members]
        return chats

    async def delete(self):
        await objects.delete((await objects.get(PgUser, id=self.id())), recursive=True, delete_nullable=True)

    @classmethod
    async def search_users(cls, username: str, offset: int = 0, limit: int = 10) -> Iterable["User"]:
        users = await objects.execute(
            PgUser.select().where(
                (PgUser.username % f'%{username}%') &
                ~(PgUser.is_bot)
            ).offset(offset).limit(limit)
        )

        return [User.from_db_model(el) for el in users]
    
    @classmethod
    async def search_bots(cls, username: str, offset: int = 0, limit: int = 10) -> Iterable["User"]:
        users = await objects.execute(
            PgUser.select().where(
                (PgUser.username % f'%{username}%') &
                (PgUser.is_bot)
            ).offset(offset).limit(limit)
        )

        return [User.from_db_model(el) for el in users]

    @classmethod
    async def get_by_id(cls, id: int) -> "User":
        user = await objects.get(PgUser, id=id)
        return cls.from_db_model(user)

    @staticmethod
    def from_db_model(db_model: PgUser) -> "User":
        return User(
            id=db_model.id,
            username=db_model.username,
            password=db_model.password,
            email=db_model.email,
            is_bot=db_model.is_bot,
            creator_id=db_model.creator_id
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
    async def new(cls, name: str, owner: User):
        pg_chat = await objects.create(
            PgChat,
            name=name,
            owner_id=owner.id()
        )

        chat = Chat.from_db_model(pg_chat)
        chat_member = await chat.add_member(owner)
        await chat_member.set_permissions(ChatMemberPermissions(True, True, True))

        return chat

    async def send_message(self, sender: User, content: str, files: List[str]) -> "ChatMessage":
        pg_message = await objects.create(
            PgChatMessage,
            sender_id=sender.id(),
            content=content,
            chat_id=self.id(),
            files=ChatMessage.FILES_SPLITTER.join(files)
        )

        message = ChatMessage.from_db_model(pg_message)

        event_payload = await NewMessage(message).to_dict()
        await self.send_event_to_members(event_payload)

        return message
    
    async def set_name(self, value: str):
        await objects.execute(PgChat.update(name=value).where(PgChat.id == self.id()))
        self._name = value

    async def owner(self) -> User:
        return await User.get_by_id(self._owner_id)
    
    async def set_owner(self, user: User):
        await objects.execute(PgChat.update(owner_id=user.id()).where(PgChat.id == self.id()))
    
    async def members(self) -> Iterable["ChatMember"]:
        members = await objects.execute(PgChatMember.filter((PgChatMember.chat_id == self.id())))
        return [ChatMember.from_db_model(el) for el in members]
    
    async def add_member(self, user: User) -> "ChatMember":
        pg_member = await objects.create(PgChatMember, user_id=user.id(), chat_id=self.id())
        member = ChatMember.from_db_model(pg_member)

        event_payload = await MemberAdded(member).to_dict()
        await self.send_event_to_members(event_payload)
        return member

    async def delete(self):
        await objects.delete((await objects.get(PgChat, id=self.id())), recursive=True)

    async def messages(self, offset: int, limit: int) -> Iterable["ChatMessage"]:
        messages = await objects.execute(
            PgChatMessage.select().where(
                (PgChatMessage.chat_id == self.id())
            ).order_by(PgChatMessage.id.desc()).offset(offset).limit(limit)
        )

        return [ChatMessage.from_db_model(el) for el in messages]
    
    async def send_event_to_members(self, event_payload: dict):
        for chat_member in await self.members():
            user = await chat_member.user()
            await Event.new(user, event_payload)

    @classmethod
    async def get_by_id(cls, id: int) -> "Chat":
        chat = await objects.get(PgChat, id=id)
        return cls.from_db_model(chat)
    
    @staticmethod
    def from_db_model(db_model: PgChat):
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
    
    async def chat(self) -> Chat:
        return await Chat.get_by_id(self._chat_id)
    
    async def user(self) -> User:
        return await User.get_by_id(self._user_id)
    
    def permissions(self) -> ChatMemberPermissions:
        return self._permissions
    
    async def set_permissions(self, value: ChatMemberPermissions):
        await objects.execute(
            PgChatMember.update(
                can_write = value.can_write,
                can_add_members = value.can_add_members,
                can_kick_members = value.can_kick_members
            ).where(
                PgChatMember.id == self.id()
            )
        )
        
        self._permissions = value

    async def delete(self):
        chat = await self.chat()
        event_payload = await MemberKicked(await self.user(), chat).to_dict()
        await objects.delete(await objects.get(PgChatMember, id=self.id()), recursive=True)
        await chat.send_event_to_members(event_payload)

    @classmethod
    async def get_by_chat_and_user(cls, chat_id: int, user_id: int):
        member = await objects.get(PgChatMember, chat_id=chat_id, user_id=user_id)
        return ChatMember.from_db_model(member)

    @staticmethod
    def from_db_model(db_model: PgChatMember) -> "ChatMember":
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
    
    async def chat(self) -> Chat:
        return await Chat.get_by_id(self._chat_id)
    
    async def sender(self) -> User:
        return await User.get_by_id(self._sender_id)
    
    def content(self) -> str:
        return self._content
    
    def file_names(self) -> Iterable[str]:
        return self._files
    
    async def set_content(self, value: str):
        await objects.execute(PgChatMessage.update(content=value).where(PgChatMessage.id == self.id()))
        self._content = value
    
    def timestamp(self) -> float:
        return self._created_timestamp
    
    @staticmethod
    def from_db_model(db_model: PgChatMessage):
        return ChatMessage(
            db_model.id,
            db_model.sender_id,
            db_model.chat_id,
            db_model.content,
            db_model.created_timestamp,
            db_model.files.split(ChatMessage.FILES_SPLITTER)
        )
    
    @classmethod
    async def get_by_id(cls, id: int) -> "ChatMessage":
        message = await objects.get(PgChatMessage, id=id)
        return cls.from_db_model(message)


class Event(IEvent):
    def __init__(self, id: int, receiver_id: int, is_read: bool, timestamp: float, payload: dict) -> None:
        self._id = id
        self._receiver_id = receiver_id
        self._is_read = is_read
        self._timestamp = timestamp
        self._payload = payload
    
    @classmethod
    async def new(cls, receiver: IUser, payload: dict) -> IEvent:
        str_payload = dumps(payload)
        
        new_event = await objects.create(
                PgEvent,
                user_id=receiver.id(),
                payload=str_payload
            )
        
        return Event.from_db_model(new_event)

    def id(self) -> int:
        return self._id

    async def receiver(self) -> User:
        user = await objects.get(PgUser, id=self._receiver_id)
        return User.from_db_model(user)
    
    def is_read(self) -> bool:
        return self._is_read
    
    def create_timestamp(self) -> float:
        return self._timestamp

    def payload(self) -> dict:
        return self._payload
    
    async def mark_as_read(self):
        await objects.execute(PgEvent.update(is_read=True).where(PgEvent.id == self.id()))

    @staticmethod
    def from_db_model(db_model: PgEvent):
        return Event(
            db_model.id,
            db_model.user_id,
            db_model.is_read,
            db_model.created_timestamp,
            loads(db_model.payload)
        )