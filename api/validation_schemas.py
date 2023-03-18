CREATE_USER = {
    'type': 'object',
    'properties': {
        'username': {'type': 'string', 'minLength': 1, 'maxLength': 100},
        'password': {'type': 'string', 'minLength': 4, 'maxLength': 100},
        'email': {'type': 'string', 'format': 'email', "minLength": 6, "maxLength": 127, "pattern": "^\\S+@\\S+\\.\\S+$"},
        'icon_file': {'type': ["string", "null"], 'minLength': 4, 'maxLength': 400, 'default': None}
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
        'username': {'type': 'string', 'minLength': 1, 'maxLength': 100},
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
        'username': {'type': 'string', 'minLength': 1, 'maxLength': 100},
        'email': {'type': 'string', 'format': 'email', "minLength": 6, "maxLength": 127, "pattern": "^\\S+@\\S+\\.\\S+$"},
        'icon_file': {'type': ["string", "null"], 'minLength': 4, 'maxLength': 400, 'default': None}
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
        'username': {'type': 'string', 'minLength': 1, 'maxLength': 100},
        'icon_file': {'type': 'string', 'minLength': 4, 'maxLength': 400, 'default': None}
    },
    'required': [
        'username'
    ]
}

CREATE_CHAT = {
    'type': 'object',
    'properties': {
        'name': {'type': 'string', 'minLength': 1, 'maxLength': 100},
        'icon_file': {'type': ["string", "null"], 'minLength': 4, 'maxLength': 400, 'default': None}
    },
    'required': [
        'name'
    ]
}

PATCH_CHAT = {
    'type': 'object',
    'properties': {
        'name': {'type': 'string', 'minLength': 1, 'maxLength': 100},
        'owner_id': {'type': 'integer'},
        'icon_file': {'type': ["string", "null"], 'minLength': 4, 'maxLength': 400, 'default': None}
    }
}

ADD_MEMBER = {
    'type': 'object',
    'properties': {
        'user_id': {'type': 'integer'}
    },
    'required': [
        'user_id'
    ]
}


PATCH_MEMBER_PERMISSIONS = {
    'type': 'object',
    'properties': {
        'can_write': {'type': 'boolean'},
        'can_add_members': {'type': 'boolean'},
        'can_kick_members': {'type': 'boolean'}
    }
}

SEND_MESSAGE = {
    'type': 'object',
    'properties': {
        'content': {'type': 'string', 'minLength': 1, 'maxLength': 4000},
        'files': {'type': 'array', 'minItems': 0, 'maxItems': 10, "items": {
            "type": "string",
            "maxLength": 128,
            "minLength": 10}
        }
    },

    'required': [
        'content',
        'files'
    ]
}

UPDATE_MESSAGE = {
    'type': 'object',
    'properties': {
        'content': {'type': 'string', 'minLength': 1, 'maxLength': 4000},
        'files': {'type': 'array', 'minItems': 0, 'maxItems': 10, "items": {
            "type": "string",
            "maxLength": 128,
            "minLength": 10}
        }
    }
}

READ_EVENTS = {
    'type': 'object',
    'properties': {
        'ids': {
            'type': 'array',
            'minItems': 1,
            'maxItems': 256,
            "items": {
                "type": "integer",
            }
        }
    },
    'required': ['ids']
}
