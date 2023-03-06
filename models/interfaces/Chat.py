from typing import Iterable, TYPE_CHECKING, List
from .User import User

if TYPE_CHECKING:
    from .ChatMember import ChatMember
    from .ChatMessage import ChatMessage

class Chat:
    """
    Интерфейс модели чата
    """

    @classmethod
    async def new(cls, name: str, owner: User):
        raise NotImplementedError()

    def id(self) -> int:
        raise NotImplementedError()
    
    def name(self) -> str:
        raise NotImplementedError()
    
    async def send_message(self, sender: User, content: str, files: List[str]) -> ChatMessage:
        raise NotImplementedError()
    
    async def set_name(self, value: str):
        raise NotImplementedError()

    async def owner(self) -> User:
        raise NotImplementedError()
    
    async def set_owner(self, user: User):
        raise NotImplementedError()
    
    async def members(self) -> Iterable["ChatMember"]:
        raise NotImplementedError()
    
    async def messages(self, offset: int, limit: int) -> Iterable["ChatMessage"]:
        raise NotImplementedError()

    async def delete(self):
        raise NotImplementedError()

    @classmethod
    async def get_by_id(cls, id: int) -> "Chat":
        raise NotImplementedError()
    
    async def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'name': self.name(),
            'owner': await (await self.owner()).to_dict()
        }