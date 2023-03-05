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
    
    def chat(self) -> Chat:
        raise NotImplementedError()

    def sender(self) -> User:
        raise NotImplementedError()

    def permissions(self) -> ChatMemberPermissions:
        raise NotImplementedError()