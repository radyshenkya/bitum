from dataclasses import dataclass
from .User import User
from .Chat import Chat

@dataclass
class ChatMemberPermissions:
    can_write: bool
    can_add_members: bool
    can_kick_members: bool

class ChatMember:
    """
    Модель участника чата
    """

    def id(self) -> int:
        raise NotImplementedError()
    
    async def chat(self) -> Chat:
        raise NotImplementedError()

    async def user(self) -> User:
        raise NotImplementedError()

    def permissions(self) -> ChatMemberPermissions:
        raise NotImplementedError()
    
    async def set_permissions(self, new_permissions: ChatMemberPermissions):
        raise NotImplementedError()
    
    async def delete(self):
        raise NotImplementedError()
    
    @classmethod
    async def get_by_chat_and_user(cls, chat_id: int, user_id: int):
        raise NotImplementedError()

    async def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'user': await (await self.user()).to_dict(),
            'chat': await (await self.chat()).to_dict()
        }