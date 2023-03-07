from functools import wraps
from http import HTTPStatus
from flask import Response, jsonify

from models.interfaces import ApiError


def ok(data=None) -> Response:
    if data is None:
        return jsonify({'ok': True})

    return jsonify({'ok': True, 'data': data})


def only_user(function):
    @wraps(function)
    def wrapper(*args, **kwargs):
        try:
            assert 'user' in kwargs.keys()
            assert not kwargs['user'].is_bot()
            return function(*args, **kwargs)

        except AssertionError as e:
            raise ApiError(HTTPStatus.FORBIDDEN,
                           'Bots can not use this method.', e)

        except Exception as e:
            raise e

    return wrapper


def only_bot(function):
    @wraps(function)
    def wrapper(*args, **kwargs):
        try:
            assert 'user' in kwargs.keys()
            assert kwargs['user'].is_bot()
            return function(*args, **kwargs)

        except AssertionError as e:
            raise ApiError(HTTPStatus.FORBIDDEN,
                           'Users can not use this method.', e)

        except Exception as e:
            raise e

    return wrapper
