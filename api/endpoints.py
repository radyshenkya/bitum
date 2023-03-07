from hashlib import sha1
from http import HTTPStatus
from models.interfaces import ApiError
from models.postgres.models import User, Chat, ChatMember, ChatMessage
from . import validation_schemas
from .jwt import get_user_from_jwt, generate_jwt, JWT_TOKEN_COOKIE_NAME
from .util import ok, only_bot, only_user
from .error_handlers import bind as bind_errors
from config import API_FILES_SAVE_PATH, MAX_FILE_SIZE_IN_BYTES

from jsonschema import ValidationError
from werkzeug import exceptions
from flask_expects_json import expects_json
from flask import Blueprint, make_response, request, jsonify, send_from_directory

api = Blueprint('api', __name__)
bind_errors(api)


# /user
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


@api.route('/user/request_reset_password/<string:username>', methods=['POST'], strict_slashes=False)
@only_user
def request_reset_password(username: str):
    raise NotImplementedError()


@api.route('/user/reset_password/<string:code>', methods=['POST'], strict_slashes=False)
@expects_json(validation_schemas.RESET_PASSWORD)
@only_user
def reset_password(code: str):
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

    token = generate_jwt(user)
    resp = ok({'token': token})
    resp.set_cookie(JWT_TOKEN_COOKIE_NAME, token)

    return resp


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


@api.route('/user/search')
def search_users():
    username = request.args.get('username', '')
    offset = int(request.args.get('offset', 0))
    limit = min(int(request.args.get('limit', 10)), 50)

    users = User.search_users(username, offset, limit)
    return ok([el.to_dict() for el in users])


# /bot
@api.route('/bot', methods=['POST'], strict_slashes=False)
@expects_json(validation_schemas.CREATE_BOT)
@get_user_from_jwt
@only_user
def create_bot(user: User):
    return ok(User.new_bot(request.json['username'], user).to_dict())


@api.route('/bot/<int:bot_id>', methods=['DELETE'], strict_slashes=False)
@get_user_from_jwt
@only_user
@ApiError.wrap_exception(AssertionError, HTTPStatus.NOT_FOUND, 'This user does not own this bot')
def delete_bot(bot_id: int, user: User):
    bot = User.get_by_id(bot_id)
    assert bot.is_bot()
    assert bot.creator().id() == user.id()

    bot.delete()
    return ok()


@api.route('/bots', methods=['GET'], strict_slashes=False)
@get_user_from_jwt
@only_user
def get_bots(user: User):
    return ok([el.to_dict() for el in user.owned_bots()])


@api.route('/bot/<int:bot_id>/token', methods=['POST'], strict_slashes=False)
@get_user_from_jwt
@only_user
@ApiError.wrap_exception(AssertionError, HTTPStatus.NOT_FOUND, 'This user does not own this bot')
def get_bot_token(bot_id: int, user: User):
    bot = User.get_by_id(bot_id)
    assert bot.is_bot()
    assert bot.creator().id() == user.id()

    return ok({'token': generate_jwt(bot)})


@api.route('/bot/search', methods=['GET'], strict_slashes=False)
def search_bots():
    username = request.args.get('username', '')
    offset = int(request.args.get('offset', 0))
    limit = min(int(request.args.get('limit', 10)), 50)

    bots = User.search_bots(username, offset, limit)
    return ok([el.to_dict() for el in bots])


# /files
@api.route('/files', methods=['POST'], strict_slashes=False)
@get_user_from_jwt
@ApiError.wrap_exception(AssertionError, HTTPStatus.BAD_REQUEST, f'Files bigger than {MAX_FILE_SIZE_IN_BYTES} bytes is not supported')
def upload_file(user: User):
    files = []

    for name, file in request.files.items():
        content = file.stream.read()
        assert len(content) <= MAX_FILE_SIZE_IN_BYTES
        file_name = f'{sha1(content).hexdigest()}.{file.filename.split(".")[-1]}'

        with open(f"{API_FILES_SAVE_PATH}/{file_name}", 'wb') as f:
            f.write(content)
            f.flush()

        files.append(file_name)

    return ok(files)


@api.route('/files/<path:path>', methods=['GET'], strict_slashes=False)
def server_files(path):
    return send_from_directory(API_FILES_SAVE_PATH, path)
