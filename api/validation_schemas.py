CREATE_USER = {
    'type': 'object',
    'properties': {
        'username': {'type': 'string', 'minLength': 3, 'maxLength': 100},
        'password': {'type': 'string', 'minLength': 3, 'maxLength': 100},
        'email': {'type': 'string', 'format': 'email', "minLength": 6, "maxLength": 127, "pattern": "^\\S+@\\S+\\.\\S+$"}
    },
    'required': [
        'username',
        'password',
        'email'
    ]
}