from ..interfaces import ChatMember
from .EventType import EventType


class MemberAdded(EventType):
    def __init__(self, chat_member: ChatMember) -> None:
        self._member = chat_member

    def data_to_dict(self) -> dict:
        return self._member.to_dict()
