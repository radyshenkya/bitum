from ..interfaces import User, Chat
from .EventType import EventType


class MemberKicked(EventType):
    def __init__(self, user: User, chat: Chat) -> None:
        self._user = user
        self._chat = chat

    async def data_to_dict(self) -> dict:
        return {
            "user": await self._user.to_dict(),
            "chat": await self._chat.to_dict()
        }
