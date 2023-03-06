from .User import User

class Event:
    """
    Интерфейс модели события
    """

    def id(self) -> int:
        raise NotImplementedError()
    
    async def receiver(self) -> User:
        raise NotImplementedError()
    
    def is_read(self) -> bool:
        raise NotImplementedError()
    
    def create_timestamp(self) -> float:
        raise NotImplementedError()

    def payload(self) -> dict:
        raise NotImplementedError()
    
    async def mark_as_read(self):
        raise NotImplementedError()