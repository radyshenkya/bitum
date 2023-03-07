from typing import Iterable
from .Chat import Chat
from .User import User


class ChatMessage:
    """
    Интерфейс модели сообщения из чата
    """

    def id(self) -> int:
        raise NotImplementedError()

    def chat(self) -> Chat:
        raise NotImplementedError()

    def sender(self) -> User:
        raise NotImplementedError()

    def content(self) -> str:
        raise NotImplementedError()

    def file_names(self) -> Iterable[str]:
        raise NotImplementedError()

    def set_content(self, value: str):
        raise NotImplementedError()

    def timestamp(self) -> float:
        raise NotImplementedError()

    def delete(self):
        raise NotImplementedError()

    @classmethod
    def get_by_id(cls, id: int) -> "ChatMessage":
        raise NotImplementedError()

    def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'sender': self.sender().to_dict(),
            'chat': self.chat().to_dict(),
            'content': self.content(),
            'files': [el for el in self.file_names()]
        }
