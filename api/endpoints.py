from hashlib import sha1
from http import HTTPStatus
from models.interfaces import ApiError
from models.postgres.models import User, Chat, ChatMember, ChatMessage, ChatMemberPermissions, Event
from . import validation_schemas
from .jwt import get_user_from_jwt, generate_jwt, JWT_TOKEN_COOKIE_NAME
from .util import ok, only_user
from .error_handlers import bind as bind_errors
from config import API_FILES_SAVE_PATH, MAX_FILE_SIZE_IN_BYTES

from flask_cors import cross_origin
from flask_expects_json import expects_json
from flask import Blueprint, request, send_from_directory

api = Blueprint('api', __name__)
bind_errors(api)


# /user
@api.route('/user', methods=['GET'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
def get_my_user(user: User):
    return ok(user.to_dict())


@api.route('/user', methods=['POST'], strict_slashes=False)
@cross_origin()
@expects_json(validation_schemas.CREATE_USER)
def new_user():
    json_request = request.json
    user = User.new(json_request['username'],
                    json_request['password'], json_request['email'])

    return ok(user.to_dict())


@api.route('/user/<int:user_id>', methods=['GET'], strict_slashes=False)
@cross_origin()
def get_user_by_id(user_id: int):
    user = User.get_by_id(user_id)
    return ok(user.to_dict())


@api.route('/user/<string:username>', methods=['GET'], strict_slashes=False)
@cross_origin()
def get_user_by_username(username: str):
    user = User.get_by_username(username)
    return ok(user.to_dict())


@api.route('/user/request_reset_password/<string:username>', methods=['POST'], strict_slashes=False)
@cross_origin()
@only_user
def request_reset_password(username: str):
    raise NotImplementedError()


@api.route('/user/reset_password/<string:code>', methods=['POST'], strict_slashes=False)
@cross_origin()
@expects_json(validation_schemas.RESET_PASSWORD)
@only_user
def reset_password(code: str):
    raise NotImplementedError()


@api.route('/user/token', methods=['POST'], strict_slashes=False)
@cross_origin()
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
@cross_origin()
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
@cross_origin()
def search_users():
    username = request.args.get('username', '')
    offset = int(request.args.get('offset', 0))
    limit = min(int(request.args.get('limit', 10)), 50)

    users = User.search_users(username, offset, limit)
    return ok([el.to_dict() for el in users])


# /bot
@api.route('/bot', methods=['POST'], strict_slashes=False)
@cross_origin()
@expects_json(validation_schemas.CREATE_BOT)
@get_user_from_jwt
@only_user
def create_bot(user: User):
    return ok(User.new_bot(request.json['username'], user).to_dict())


@api.route('/bot/<int:bot_id>', methods=['DELETE'], strict_slashes=False)
@cross_origin()
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
@cross_origin()
@get_user_from_jwt
@only_user
def get_bots(user: User):
    return ok([el.to_dict() for el in user.owned_bots()])


@api.route('/bot/<int:bot_id>/token', methods=['POST'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@only_user
@ApiError.wrap_exception(AssertionError, HTTPStatus.NOT_FOUND, 'This user does not own this bot')
def get_bot_token(bot_id: int, user: User):
    bot = User.get_by_id(bot_id)
    assert bot.is_bot()
    assert bot.creator().id() == user.id()

    return ok({'token': generate_jwt(bot)})


@api.route('/bot/search', methods=['GET'], strict_slashes=False)
@cross_origin()
def search_bots():
    username = request.args.get('username', '')
    offset = int(request.args.get('offset', 0))
    limit = min(int(request.args.get('limit', 10)), 50)

    bots = User.search_bots(username, offset, limit)
    return ok([el.to_dict() for el in bots])


# /files
@api.route('/files', methods=['POST'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@ApiError.wrap_exception(AssertionError, HTTPStatus.BAD_REQUEST, f'Files bigger than {MAX_FILE_SIZE_IN_BYTES} bytes is not supported')
def upload_file(user: User):
    files = []

    for name, file in request.files.items()[:10]:
        content = file.stream.read()
        assert len(content) <= MAX_FILE_SIZE_IN_BYTES
        file_name = f'{sha1(content).hexdigest()}.{file.filename.split(".")[-1]}'

        with open(f"{API_FILES_SAVE_PATH}/{file_name}", 'wb') as f:
            f.write(content)
            f.flush()

        files.append(file_name)

    return ok(files)


@api.route('/files/<path:path>', methods=['GET'], strict_slashes=False)
@cross_origin()
def server_files(path):
    return send_from_directory(API_FILES_SAVE_PATH, path)


# /chat
@api.route('/chat', methods=['POST'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.CREATE_CHAT)
def new_chat(user: User):
    json_request = request.json
    new_chat = Chat.new(json_request['name'], user)
    return ok(new_chat.to_dict())


@api.route('/chat/<int:chat_id>', methods=['PATCH'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.PATCH_CHAT)
def patch_chat(chat_id: int, user: User):
    json_request = request.json
    chat = Chat.get_by_id(chat_id)
    chat_owner = chat.owner()
    assert chat_owner.id() == user.id()

    new_owner = User.get_by_id(json_request.get('owner_id', chat_owner.id()))

    chat.set_name(json_request.get('name', chat.name()))
    chat.set_owner(new_owner)

    return ok(chat.to_dict())


@api.route('/chat/<int:chat_id>', methods=['DELETE'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not delete this chat')
def delete_chat(chat_id: int, user: User):
    chat = Chat.get_by_id(chat_id)
    assert chat.owner().id() == user.id()

    chat.delete()
    return ok()


@api.route('/chats', methods=['GET'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
def get_chats(user: User):
    chats = user.chats()
    return ok([el.to_dict() for el in chats])


# /chat/ /member
@api.route('/chat/<int:chat_id>/member', methods=["POST"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.ADD_MEMBER)
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not add members to this chat')
def add_chat_member(chat_id: int, user: User):
    chat = Chat.get_by_id(chat_id)
    user_member = ChatMember.get_by_chat_and_user(chat, user)

    assert user_member.permissions().can_add_members

    user_to_add = User.get_by_id(request.json['user_id'])
    chat_member = chat.add_member(user_to_add)

    return ok(chat_member.to_dict())


@api.route('/chat/<int:chat_id>/member/<int:user_id>', methods=["DELETE"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not kick members in this chat')
def delete_member(chat_id: int, user_id: int, user: User):
    chat = Chat.get_by_id(chat_id)

    assert ChatMember.get_by_chat_and_user(
        chat, user).permissions().can_kick_members

    asked_user = User.get_by_id(user_id)
    chat_member = ChatMember.get_by_chat_and_user(chat, asked_user)

    chat_member.delete()

    return ok()


@api.route('/chat/<int:chat_id>/member/<int:user_id>', methods=["GET"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
def get_chat_member_info(chat_id: int, user_id: int, user: User):
    chat = Chat.get_by_id(chat_id)

    ChatMember.get_by_chat_and_user(chat, user)

    asked_user = User.get_by_id(user_id)
    chat_member = ChatMember.get_by_chat_and_user(chat, asked_user)

    return ok(chat_member.to_dict())


@api.route('/chat/<int:chat_id>/member/<int:user_id>', methods=["PATCH"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.PATCH_MEMBER_PERMISSIONS)
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not edit members in this chat')
def patch_member_permissions(chat_id: int, user_id: int, user: User):
    chat = Chat.get_by_id(chat_id)

    assert chat.owner().id() == user.id()

    asked_user = User.get_by_id(user_id)
    chat_member = ChatMember.get_by_chat_and_user(chat, asked_user)

    chat_member.set_permissions(ChatMemberPermissions(
        request.json.get('can_write',
                         chat_member.permissions().can_write),
        request.json.get('can_add_members',
                         chat_member.permissions().can_add_members),
        request.json.get('can_kick_members',
                         chat_member.permissions().can_kick_members)
    ))

    return ok(chat_member.to_dict())


# /chat/ /message
@api.route('/chat/<int:chat_id>/message', methods=["POST"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.SEND_MESSAGE)
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not send messages in this chat')
def send_message(chat_id: int, user: User):
    chat = Chat.get_by_id(chat_id)

    assert ChatMember.get_by_chat_and_user(chat, user).permissions().can_write

    message = chat.send_message(
        user,
        request.json["content"],
        request.json["files"]
    )

    return ok(message.to_dict())


@api.route('/chat/<int:chat_id>/message/<int:message_id>', methods=["PATCH"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.UPDATE_MESSAGE)
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not edit this message')
def patch_message(chat_id: int, message_id: int, user: User):
    message = ChatMessage.get_by_id(message_id)

    assert message.sender().id() == user.id()

    message.set_content(request.json.get('content', message.content()))
    message.set_files(request.json.get('files', message.files()))

    return ok(message.to_dict())


@api.route('/chat/<int:chat_id>/message/<int:message_id>', methods=["DELETE"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.UPDATE_MESSAGE)
@ApiError.wrap_exception(AssertionError, HTTPStatus.FORBIDDEN, f'You can not delete this message')
def delete_message(chat_id: int, message_id: int, user: User):
    message = ChatMessage.get_by_id(message_id)
    chat = Chat.get_by_id(chat_id)

    assert message.sender().id() == user.id() or chat.owner().id() == user.id()

    message.delete()
    return ok()


@api.route('/chat/<int:chat_id>/messages', methods=["GET"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
def get_messages(chat_id: int, user: User):
    chat = Chat.get_by_id(chat_id)
    ChatMember.get_by_chat_and_user(chat, user)

    offset = int(request.args.get('offset', 0))
    limit = min(int(request.args.get('limit', 10)), 50)

    messages = chat.messages(offset, limit)

    return ok([el.to_dict() for el in messages])


# /event
@api.route('/events', methods=["GET"], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
def get_events(user: User):
    return ok([el.to_dict() for el in user.get_unread_events()])


@api.route('/events', methods=['DELETE'], strict_slashes=False)
@cross_origin()
@get_user_from_jwt
@expects_json(validation_schemas.READ_EVENTS)
def read_events(user: User):
    user_events_ids = set([el.id() for el in list(user.get_unread_events())])
    closed = []

    for event_id in request.json['ids']:
        if not event_id in user_events_ids:
            continue

        event = Event.get_by_id(event_id)
        event.mark_as_read()

        closed.append(event_id)
        user_events_ids.remove(event_id)

    return ok({"read_event_ids": closed})
