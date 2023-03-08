from .User import User


class Event:
    """
    Интерфейс модели события
    """

    def id(self) -> int:
        raise NotImplementedError()

    def receiver(self) -> User:
        raise NotImplementedError()

    def is_read(self) -> bool:
        raise NotImplementedError()

    def create_timestamp(self) -> float:
        raise NotImplementedError()

    def payload(self) -> dict:
        raise NotImplementedError()

    def mark_as_read(self):
        raise NotImplementedError()

    @classmethod
    def get_by_id(cls, id: int) -> "Event":
        raise NotImplementedError()

    def to_dict(self) -> dict:
        return {
            'id': self.id(),
            'user': self.receiver().to_dict(),
            'payload': self.payload()
        }
