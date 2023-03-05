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

    def owner(self) -> User:
        raise NotImplementedError()

    def content(self) -> str:
        raise NotImplementedError()
    
    def file_names(self) -> Iterable[str]:
        raise NotImplementedError()
    
    def set_content(self, value: str):
        raise NotImplementedError()
    
    def timestamp(self) -> float:
        raise NotImplementedError()