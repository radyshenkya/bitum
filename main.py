import logging
from flask import Flask
from api.endpoints import api

app = Flask(__name__)

# Blueprints
app.register_blueprint(api, url_prefix='/api')


# logging
file_logger = logging.FileHandler('log.txt')
file_logger.setLevel(logging.INFO)
app.logger.addHandler(file_logger)


@app.route('/')
@app.route('/index')
def index():
    return "Aboba!"


if __name__ == "__main__":
    app.run(port=8080, host='127.0.0.1')
