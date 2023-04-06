import os
from flask import Blueprint, send_from_directory

frontend = Blueprint('frontend', __name__,
                     static_folder='bitum-frontend/dist')

@frontend.route('/', defaults={'p': ''}, methods=['GET'])
@frontend.route("/<path:p>", methods=['GET'])
def get_frontend(p: str):
    if p != "" and os.path.exists(frontend.static_folder + '/' + p):
        return send_from_directory(frontend.static_folder, p)
    else:
        return send_from_directory(frontend.static_folder, 'index.html')
