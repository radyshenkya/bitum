from os import environ
import time
import peewee
import dotenv
import peewee_async

dotenv.load_dotenv()

database = peewee_async.PostgresqlDatabase(
    environ["DB_NAME"],
    user=environ["DB_USER"],
    password=environ["DB_PASSWORD"],
    host=environ["DB_HOST"],
    port=int(environ["DB_PORT"])
)

class BaseModel(peewee.Model):
    class Meta:
        database = database


class PgUser(BaseModel):
    username = peewee.CharField(max_length=200, unique=True)
    email = peewee.CharField(max_length=200, null=True)
    password = peewee.CharField(max_length=400, null=True)
    is_bot = peewee.BooleanField(default=False)
    creator = peewee.ForeignKeyField('self', null=True, on_delete='CASCADE')


class PgChat(BaseModel):
    name = peewee.CharField(max_length=200)
    owner = peewee.ForeignKeyField(PgUser, backref='chats', null=True)


class PgChatMember(BaseModel):
    user = peewee.ForeignKeyField(PgUser, backref='user_in_chat', on_delete='CASCADE')
    chat = peewee.ForeignKeyField(PgChat, backref='chat_members', on_delete='CASCADE')
    can_write = peewee.BooleanField(default=True)
    can_add_members = peewee.BooleanField(default=True)
    can_kick_members = peewee.BooleanField(default=False)

    class Meta:
        database = database
        constraints = [
            peewee.SQL('UNIQUE (user_id, chat_id)')
        ]

class PgChatMessage(BaseModel):
    sender = peewee.ForeignKeyField(PgUser, backref='user_messages', on_delete='CASCADE')
    chat = peewee.ForeignKeyField(PgChat, backref='chat_messages', on_delete='CASCADE')
    content = peewee.TextField(null=False)
    created_timestamp = peewee.FloatField(default=time.time)
    files = peewee.TextField(default='')


class PgEvent(BaseModel):
    user = peewee.ForeignKeyField(PgUser, backref='events', on_delete='CASCADE')
    created_timestamp = peewee.FloatField(default=time.time)
    is_read = peewee.BooleanField(default=False)
    payload = peewee.TextField(null=False)


database.create_tables([PgUser, PgChat, PgChatMember, PgChatMessage, PgEvent])

database.set_allow_sync(False)
objects = peewee_async.Manager(database)