from endpoints import api

from werkzeug import exceptions
from flask import jsonify, make_response
from jsonschema import ValidationError

from models.interfaces import ModelError

@api.errorhandler(400)
def bad_request(err):
    if isinstance(err.description, ValidationError):
        return make_response(jsonify({'ok': False, 'error': {'code': 400, 'message': err.description.message}}))

    return err

@api.errorhandler(500)
def model_error_wrapper(err: exceptions.InternalServerError):
    if isinstance(err.original_exception, ModelError):
        return make_response(jsonify({'ok': False, 'error': {'code': err.original_exception.status_code, 'message': err.original_exception.message}}))

    return err