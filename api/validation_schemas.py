CREATE_USER = {
    'type': 'object',
    'properties': {
        'username': {'type': 'string', 'minLength': 3, 'maxLength': 100},
        'password': {'type': 'string', 'minLength': 4, 'maxLength': 100},
        'email': {'type': 'string', 'format': 'email', "minLength": 6, "maxLength": 127, "pattern": "^\\S+@\\S+\\.\\S+$"}
    },
    'required': [
        'username',
        'password',
        'email'
    ]
}

CREATE_USER_TOKEN = {
    'type': 'object',
    'properties': {
        'username': {'type': 'string', 'minLength': 3, 'maxLength': 100},
        'password': {'type': 'string', 'minLength': 4, 'maxLength': 100}
    },
    'required': [
        'username',
        'password'
    ]
}

PATCH_USER = {
    'type': 'object',
    'properties': {
        'username': {'type': 'string', 'minLength': 3, 'maxLength': 100},
        'email': {'type': 'string', 'format': 'email', "minLength": 6, "maxLength": 127, "pattern": "^\\S+@\\S+\\.\\S+$"}
    }
}


RESET_PASSWORD = {
    'type': 'object',
    'properties': {
        'password': {'type': 'string', 'minLength': 4, 'maxLength': 100}
    },
    'required': [
        'password'
    ]
}

CREATE_BOT = {
    'type': 'object',
    'properties': {
        'username': {'type': 'string', 'minLength': 3, 'maxLength': 100}
    },
    'required': [
        'username'
    ]
}
