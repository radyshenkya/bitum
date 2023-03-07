from flask import Response, jsonify

def ok(data) -> Response:
    return jsonify({'ok': True, 'data': data})