from flask import Flask
from api.endpoints import api

app = Flask(__name__)
app.register_blueprint(api, url_prefix='/api')

@app.route('/')
@app.route('/index')
async def index():
    return "Aboba!"

if __name__ == "__main__":
    app.run(port=8080, host='127.0.0.1')