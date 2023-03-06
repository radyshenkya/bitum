from typing import Iterable
from .Chat import Chat
from .User import User

class ChatMessage:
    """
    Интерфейс модели сообщения из чата
    """
    
    def id(self) -> int:
        raise NotImplementedError()

    async def chat(self) -> Chat:
        raise NotImplementedError()

    async def sender(self) -> User:
        raise NotImplementedError()

    def content(self) -> str:
        raise NotImplementedError()
    
    def file_names(self) -> Iterable[str]:
        raise NotImplementedError()
    
    async def set_content(self, value: str):
        raise NotImplementedError()
    
    def timestamp(self) -> float:
        raise NotImplementedError()
    
    async def delete(self):
        raise NotImplementedError()
    
    @classmethod
    async def get_by_id(cls, id: int) -> "ChatMessage":
        raise NotImplementedError()

    async def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'sender': await (await self.user()).to_dict(),
            'chat': await (await self.chat()).to_dict(),
            'content': self.content(),
            'files': [el for el in self.file_names()]
        }