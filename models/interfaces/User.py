from typing import Iterable, TYPE_CHECKING, Union

if TYPE_CHECKING:
    from .Event import Event
    from .Chat import Chat


class User:
    """
    Интерфейс модели пользователя
    """

    @classmethod
    def new(cls, username: str, password: str, email: str, icon_file: Union[str, None]) -> "User":
        raise NotImplementedError()

    @classmethod
    def new_bot(cls, username: str, creator: "User") -> "User":
        raise NotImplementedError()

    def id(self) -> int:
        raise NotImplementedError()

    def username(self) -> str:
        raise NotImplementedError()

    def email(self) -> str:
        raise NotImplementedError()

    def icon(self) -> Union[str, None]:
        raise NotImplementedError()

    def set_icon(self, value: Union[str, None]):
        raise NotImplementedError()

    def set_email(self, value: str):
        raise NotImplementedError()

    def created_timestamp(self) -> float:
        raise NotImplementedError()

    def compare_password(self, raw_password: str) -> bool:
        """Сравнить пароль с паролем пользователя"""
        raise NotImplementedError()

    def set_password(self, value: str):
        raise NotImplementedError()

    def get_unread_events(self) -> Iterable["Event"]:
        raise NotImplementedError()

    def is_bot(self) -> bool:
        raise NotImplementedError()

    def owned_bots(self) -> Iterable["User"]:
        raise NotImplementedError()

    def creator(self) -> Union["User", None]:
        raise NotImplementedError()

    def chats(self) -> Iterable["Chat"]:
        raise NotImplementedError()

    def delete(self):
        raise NotImplementedError()

    def last_login_timestamp(self) -> float:
        raise NotImplementedError()

    def update_login_timestamp(self):
        raise NotImplementedError()

    @classmethod
    def search_users(cls, username: str, offset: int = 0, limit: int = 10) -> Iterable["User"]:
        raise NotImplementedError()

    @classmethod
    def search_bots(cls, username: str, offset: int = 0, limit: int = 10) -> Iterable["User"]:
        raise NotImplementedError()

    @classmethod
    def get_by_id(cls, id: int) -> "User":
        raise NotImplementedError()

    @classmethod
    def get_by_username(cls, username: str) -> "User":
        raise NotImplementedError()

    def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'username': self.username(),
            'is_bot': self.is_bot(),
            'created_at': self.created_timestamp(),
            'icon': self.icon()
        }
