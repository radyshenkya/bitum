import re

_RENAME_PATTERN = re.compile(r'(?<!^)(?=[A-Z])')


class EventType:
    @classmethod
    def name(cls):
        """Получить имя класса в стиле `snake_case`"""
        return _RENAME_PATTERN.sub('_', cls.__name__).lower()
    
    async def data_to_dict(self) -> dict:
        """Получить данные ивента словарем"""
        raise NotImplementedError()

    async def to_dict(self) -> dict:
        """Получить весь ивент словарем"""
        return {
            "type": self.name(),
            "data": await self.data_to_dict()
        }