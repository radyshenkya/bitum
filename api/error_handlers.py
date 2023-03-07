from werkzeug import exceptions
from flask import Blueprint, jsonify, make_response
from jsonschema import ValidationError

from models.interfaces.ApiError import ApiError


def validation_error_wrapper(err: exceptions.BadRequest):
    if isinstance(err.description, ValidationError):
        return make_response(jsonify({'ok': False, 'error': {'code': 400, 'message': err.description.message}}))

    return err


def api_error_wrapper(err: exceptions.InternalServerError):
    if isinstance(err.original_exception, ApiError):
        return make_response(jsonify({'ok': False, 'error': {'code': err.original_exception.status_code, 'message': err.original_exception.message}}))

    return err


def bind(api: Blueprint):
    api.register_error_handler(500, api_error_wrapper)
    api.register_error_handler(400, validation_error_wrapper)
