from werkzeug import exceptions
from flask import Blueprint, jsonify, make_response
from jsonschema import ValidationError

from models.interfaces.ModelError import ModelError


def bad_request(err):
    if isinstance(err.description, ValidationError):
        return make_response(jsonify({'ok': False, 'error': {'code': 400, 'message': err.description.message}}))

    return err


def model_error_wrapper(err: exceptions.InternalServerError):
    if isinstance(err.original_exception, ModelError):
        return make_response(jsonify({'ok': False, 'error': {'code': err.original_exception.status_code, 'message': err.original_exception.message}}))

    return err


def bind(api: Blueprint):
    api.register_error_handler(500, model_error_wrapper)
    api.register_error_handler(400, bad_request)