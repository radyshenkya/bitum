from functools import wraps

class ApiError(Exception):
    def __init__(self, status_code: int, message: str, original_exception: Exception) -> None:
        self.message = message
        self.original_exception = original_exception
        self.status_code = status_code
    
    @staticmethod
    def wrap_exception(exception_class, status_code: int, message: str = "Unknown error"):
        def decorator(function):
            @wraps(function)
            def wrapper(*args, **kwargs):
                try:
                    return function(*args, **kwargs)
                except Exception as e:
                    if isinstance(e, exception_class):
                        raise ApiError(status_code, message, e)
                    else:
                        raise e
            
            return wrapper
        return decorator