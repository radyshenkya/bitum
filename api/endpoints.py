from http import HTTPStatus
from models.interfaces import ApiError
from models.postgres.models import User, Chat, ChatMember, ChatMessage
from . import validation_schemas
from .jwt import get_user_from_jwt, generate_jwt
from .util import ok, only_bot, only_user
from .error_handlers import bind as bind_errors

from jsonschema import ValidationError
from werkzeug import exceptions
from flask_expects_json import expects_json
from flask import Blueprint, make_response, request, jsonify

api = Blueprint('api', __name__)
bind_errors(api)


@api.route('/user', methods=['GET'], strict_slashes=False)
@get_user_from_jwt
def get_my_user(user: User):
    return ok(user.to_dict())


@api.route('/user', methods=['POST'], strict_slashes=False)
@expects_json(validation_schemas.CREATE_USER)
def new_user():
    json_request = request.json
    user = User.new(json_request['username'],
                    json_request['password'], json_request['email'])

    return ok(user.to_dict())


@api.route('/user/<int:user_id>', methods=['GET'], strict_slashes=False)
def get_user_by_id(user_id: int):
    user = User.get_by_id(user_id)
    return ok(user.to_dict())


@api.route('/user/<string:username>', methods=['GET'], strict_slashes=False)
def get_user_by_username(username: str):
    user = User.get_by_username(username)
    return ok(user.to_dict())


@api.route('/user', methods=['PATCH'], strict_slashes=False)
@expects_json(validation_schemas.PATCH_USER)
@get_user_from_jwt
@only_user
def patch_user(user: User):
    json_request = request.json
    user.set_email(json_request.get('email', user.email()))

    if 'password' in json_request.keys():
        user.set_password(json_request['password'])

    return ok(user.to_dict())


@api.route('/user/request_reset_password/<string: username>', methods=['POST'], strict_slashes=False)
def request_reset_password(username: str):
    raise NotImplementedError()


@api.route('/user/request_reset_password/<string: username>', methods=['POST'], strict_slashes=False)
def request_reset_password(username: str):
    raise NotImplementedError()


@api.route('/user/token', methods=['POST'], strict_slashes=False)
@expects_json(validation_schemas.CREATE_USER_TOKEN)
@ApiError.wrap_exception(AssertionError, HTTPStatus.UNAUTHORIZED, 'Wrong password')
def create_user_token():
    json_request = request.json
    user = User.get_by_username(json_request['username'])

    assert user.compare_password(json_request['password'])
    # TODO: Сейчас при фейле этой проверки выводит 'Wrong password', хотя это не так...
    assert not user.is_bot()

    return ok({'token': generate_jwt(user)})
