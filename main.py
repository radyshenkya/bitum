import logging
from flask import Flask
from api.endpoints import api
from frontend.endpoints import frontend

app = Flask(__name__)

# Blueprints
app.register_blueprint(api, url_prefix='/api')
app.register_blueprint(frontend, url_prefix="/")

# logging
file_logger = logging.FileHandler('log.txt')
file_logger.setLevel(logging.INFO)
app.logger.addHandler(file_logger)


if __name__ == "__main__":
    app.run(host='0.0.0.0', port='8000')
