from ..interfaces import ChatMessage
from .EventType import EventType


class NewMessage(EventType):
    def __init__(self, message: ChatMessage) -> None:
        self._message = message

    async def data_to_dict(self) -> dict:
        return await self._message.to_dict()
