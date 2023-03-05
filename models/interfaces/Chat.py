from typing import Iterable, TYPE_CHECKING
from .User import User

if TYPE_CHECKING:
    from .ChatMember import ChatMember
    from .ChatMessage import ChatMessage

class Chat:
    """
    Интерфейс модели чата
    """

    def id(self) -> int:
        raise NotImplementedError()
    
    def name(self) -> str:
        raise NotImplementedError()
    
    def set_name(self, value: str):
        raise NotImplementedError()

    def owner(self) -> User:
        raise NotImplementedError()
    
    def members(self) -> Iterable["ChatMember"]:
        raise NotImplementedError()
    
    def messages(self, starts_from: int, count: int) -> Iterable["ChatMessage"]:
        raise NotImplementedError()
    
    @classmethod
    def get_by_id(cls, id: int) -> "Chat":
        raise NotImplementedError()
    
    @classmethod
    def user_chats(cls, user: User) -> Iterable["Chat"]:
        raise NotImplementedError()