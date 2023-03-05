from typing import Iterable, Union
from json import loads, dumps
from ..interfaces import User as IUser, Chat as IChat, ChatMember as IChatMember, ChatMessage as IChatMessage, Event as IEvent
from .database import PgUser, PgChat, PgChatMember, PgChatMessage, PgEvent, objects
from bcrypt import hashpw, checkpw, gensalt

UTF_8 = 'utf-8'

class User(IUser):
    def __init__(self, id: int, username: str, db_model: PgUser, password: str | None = None, email: str | None = None, is_bot: bool = False, creator_id: int | None = None) -> None:
        self.db_model = db_model
        self._id = id
        self._username = username
        self._password = password
        self._email = email
        self._is_bot = is_bot
        self._creator_id = creator_id
    
    @staticmethod
    async def new(username: str, password: str, email: str) -> "User":
        hashed_password = hashpw(bytes(password, UTF_8), gensalt()).decode(UTF_8)
        new_user = await objects.create(
            PgUser,
            username=username,
            email=email,
            password=hashed_password
        )
        return User.from_db_model(new_user)
    
    @staticmethod
    async def new_bot(username: str, creator: "User") -> "User":
        print(creator.username())
        print(creator.db_model.username)
        new_user = await objects.create(
            PgUser,
            username=username,
            creator_id=creator.id(),
            is_bot=True,
        )
        print(creator.db_model.id)
        print(new_user.creator_id)
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
            creator_id=db_model.creator_id,
            db_model=db_model
        )


class Event(IEvent):
    def __init__(self, id: int, receiver_id: int, is_read: bool, timestamp: float, payload: dict) -> None:
        self._id = id
        self._receiver_id = receiver_id
        self._is_read = is_read
        self._timestamp = timestamp
        self._payload = payload
    
    async def new(self, receiver: IUser, payload: dict) -> IEvent:
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