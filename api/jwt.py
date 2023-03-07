from functools import wraps
from http import HTTPStatus
import json
from os import environ
from datetime import datetime, timedelta, timezone
from typing import Any, Dict

from models.interfaces import ApiError
from models.postgres.models import User

from flask import request
import jwt


JWT_EXP_HOURS = 31 * 24
JWT_HS256_SECRET = environ['JWT_SECRET']

JWT_TOKEN_COOKIE_NAME = 'api_token'


def generate_jwt(user: User) -> str:
    payload = {
        'exp': datetime.now(timezone.utc) + timedelta(hours=JWT_EXP_HOURS),
        'user_id': user.id()
    }

    return jwt.encode(payload, JWT_HS256_SECRET)


def validate_and_parse_jwt(token: str) -> Dict[str, Any]:
    payload = jwt.decode(token, JWT_HS256_SECRET, algorithms=["HS256"])
    return payload

def get_user_from_jwt(function):
    @wraps(function)
    def wrapper(*args, **kwargs):
        try:
            token = request.cookies.get(JWT_TOKEN_COOKIE_NAME, '')
            parsed = validate_and_parse_jwt(token)

            user = User.get_by_id(parsed['user_id'])
            return function(*args, **kwargs, user=user)
        
        except Exception as e:
            raise ApiError(HTTPStatus.UNAUTHORIZED, 'Unauthorized', e)
    
    return wrapper