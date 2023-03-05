from typing import Iterable, TYPE_CHECKING, Union

if TYPE_CHECKING:
    from .Event import Event


class User:
    """
    Интерфейс модели пользователя
    """

    def id(self) -> int:
        raise NotImplementedError()
    
    def username(self) -> str:
        raise NotImplementedError()

    def email(self) -> str:
        raise NotImplementedError()
    
    def set_email(self, value: str):
        raise NotImplementedError()

    def compare_password(self, raw_password: str) -> bool:
        """Сравнить пароль с паролем пользователя"""
        raise NotImplementedError()
    
    def get_unread_events(self) -> Iterable["Event"]:
        raise NotImplementedError()
        
    def is_bot(self) -> bool:
        raise NotImplementedError()

    def owned_bots(self) -> Iterable["User"]:
        raise NotImplementedError()
    
    def creator(self) -> Union["User", None]:
        raise NotImplementedError()
    
    @classmethod
    def search_users(cls, username: str) -> Iterable["User"]:
        raise NotImplementedError()
    
    @classmethod
    def search_bots(cls, username: str) -> Iterable["User"]:
        raise NotImplementedError()
    
    @classmethod
    def get_by_id(cls, id: int) -> "User":
        raise NotImplementedError()