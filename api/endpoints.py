from jsonschema import ValidationError
from models.interfaces import ModelError
from models.postgres.models import User, Chat, ChatMember, ChatMessage
from . import validation_schemas
from .util import ok

from werkzeug import exceptions
from flask_expects_json import expects_json
from flask import Blueprint, Response, make_response, request, jsonify

api = Blueprint('api', __name__)

@api.route('/user', methods=['GET'])
async def get_my_user():
    return jsonify({"data": "aboba"})

@api.route('/user', methods=['POST'])
@expects_json(validation_schemas.CREATE_USER)
async def new_user():
    json_request = request.json
    user = await User.new(json_request['username'], json_request['password'], json_request['email'])

    return ok(await user.to_dict())

# @api.route()