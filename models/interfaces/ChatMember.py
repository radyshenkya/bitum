from dataclasses import dataclass
from .User import User
from .Chat import Chat


@dataclass
class ChatMemberPermissions:
    can_write: bool
    can_add_members: bool
    can_kick_members: bool

    def to_dict(self) -> dict:
        return {
            "can_write": self.can_write,
            "can_add_members": self.can_add_members,
            "can_kick_members": self.can_kick_members
        }


class ChatMember:
    """
    Модель участника чата
    """

    def id(self) -> int:
        raise NotImplementedError()

    def chat(self) -> Chat:
        raise NotImplementedError()

    def user(self) -> User:
        raise NotImplementedError()

    def permissions(self) -> ChatMemberPermissions:
        raise NotImplementedError()

    def set_permissions(self, new_permissions: ChatMemberPermissions):
        raise NotImplementedError()

    def delete(self):
        raise NotImplementedError()

    @classmethod
    def get_by_chat_and_user(cls, chat: Chat, user: User):
        raise NotImplementedError()

    def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'user': self.user().to_dict(),
            'chat': self.chat().to_dict(),
            'permissions': self.permissions().to_dict()
        }
