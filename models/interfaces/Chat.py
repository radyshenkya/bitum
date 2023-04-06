from typing import Iterable, TYPE_CHECKING, List, Union
from .User import User

if TYPE_CHECKING:
    from .ChatMember import ChatMember
    from .ChatMessage import ChatMessage


class Chat:
    """
    Интерфейс модели чата
    """

    @classmethod
    def new(cls, name: str, owner: User, icon_file: Union[str, None] = None):
        raise NotImplementedError()

    def id(self) -> int:
        raise NotImplementedError()

    def name(self) -> str:
        raise NotImplementedError()

    def send_message(self, sender: User, content: str, files: List[str]) -> "ChatMessage":
        raise NotImplementedError()

    def set_name(self, value: str):
        raise NotImplementedError()

    def icon(self) -> Union[str, None]:
        raise NotImplementedError()

    def set_icon(self, value: Union[str, None]):
        raise NotImplementedError()

    def created_timestamp(self) -> float:
        raise NotImplementedError()

    def owner(self) -> User:
        raise NotImplementedError()

    def set_owner(self, user: User):
        raise NotImplementedError()

    def add_member(self, user: User) -> "ChatMember":
        raise NotImplementedError()

    def members(self) -> Iterable["ChatMember"]:
        raise NotImplementedError()

    def messages(self, offset: int, limit: int) -> Iterable["ChatMessage"]:
        raise NotImplementedError()

    def delete(self):
        raise NotImplementedError()

    @classmethod
    def get_by_id(cls, id: int) -> "Chat":
        raise NotImplementedError()

    def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'name': self.name(),
            'owner': self.owner().to_dict(),
            'created_at': self.created_timestamp(),
            'icon': self.icon()
        }
