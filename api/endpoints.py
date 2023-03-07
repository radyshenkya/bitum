from models.interfaces import ModelError
from models.postgres.models import User, Chat, ChatMember, ChatMessage
from . import validation_schemas
from .util import ok
from .error_handlers import bind as bind_errors

from jsonschema import ValidationError
from werkzeug import exceptions
from flask_expects_json import expects_json
from flask import Blueprint, make_response, request, jsonify

api = Blueprint('api', __name__)
bind_errors(api)


@api.route('/user', methods=['GET'])
def get_my_user():
    return ok({"aboba": 'ili bebra'})


@api.route('/user', methods=['POST'])
@expects_json(validation_schemas.CREATE_USER)
def new_user():
    json_request = request.json
    user = User.new(json_request['username'], json_request['password'], json_request['email'])

    return ok(user.to_dict())


@api.route('/user/<int:user_id>')
def get_user_by_id(user_id: int):
    user = User.get_by_id(user_id)

    return ok(user.to_dict())