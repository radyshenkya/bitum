from ..interfaces import User, Chat
from .EventType import EventType


class MemberKicked(EventType):
    def __init__(self, user: User, chat: Chat) -> None:
        self._user = user
        self._chat = chat

    def data_to_dict(self) -> dict:
        return {
            "user": self._user.to_dict(),
            "chat": self._chat.to_dict()
        }
