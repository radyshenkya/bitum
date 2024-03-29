# Введение
В **Bitum** есть *API* для написания ботов.
Работает оно по протоколу HTTP, использует в основном JSON сообщения

## Как создать бота
Для создания бота вам надо перейти на сайт проекта. После регистрации на сайте открыть вкладку **"Боты"**, и нажать там на кнопку плюса.

После создания бота он появится в списке со всеми ботами.

Токен бота можно получить кликнув на кнопку с иконкой ключа на плашке бота.

**При каждом запросе токена создаются новые токены, и старые перестают работать.**

## Как бот работает
Предполагается что бот для Bitum будет работать по такому алгоритму:
1. Бот идет на сервер и получает список [событий](#Events).
2. Бот просматривает все события, на основе которых что-то делает
3. Начинает заново с 1 пункта.

То есть бот сам должен посылать запрос на сервер для прочтения событий, и после этого что-то делать.

# Endpoints
## URL endpoint'ов
Корневой endpoint API:
```
http://example.com/api
```

Части url эндопоинта, обозначенные двумя треугольными скобками - параметры. На их местах должны стоять какие-то данные. 

## Токен
Для почти каждого запоса к API нужно добавлять хедер `Cookie` с полем `api_token`
```HTTP
Cookie: api_token=BOT_TOKEN
```


## Формат ответа
Все ответы от API приходят в формате JSON объекта.

В этом JSON объекте есть такие поля:
* `ok`: bool - `true` если запрос успешно обработан / `false` если что-то пошло не так
* `data`: object - Только если `ok` равен `true`, и ответ должен иметь какие-то данные Здесь хранятся данные ответа от сервера
* `error`: [#Error](#Error) - только если `ok` равен `false`. Хранит данные об ошибке.

Пример ответа:
```json
{
    "ok": true,
    "data": {
        ...
    }
}
```

## User
---
Методы для работы с аккаунтами пользователей. Аккаунты ботов так же считаются за пользователей.

### GET /user
Получить информацию о себе

**Ответ**
[#Structs#User](#Structs#User) - информация о вас

### GET /user/`<id>`
`id`: int - ID пользователя
Получить информацию о пользователе

**Ответ**
[#Structs#User](#Structs#User) - информация о вас

### GET /user/`<username>`
`username`: str - имя пользователя
Получить информацию о пользователе

**Ответ**
[#Structs#User](#Structs#User) - информация о вас

### GET /user/search
Поиск пользователей
**Параметры URL**
`username` - примерное имя пользователя
`limit` - лимит пользователей для выдачи
`offset` - сдвиг пользователей в выдаче

**Ответ**
Список [#Structs#User](#Structs#User) - Список найденых пользователей

## Chat
---
### POST /chat
Создать новый чат

**Тело запроса**
JSON-объект с полями:
* `name` - имя чата
* `icon_file` - название файла для иконки чата

**Ответ**
[#Structs#Chat](#Structs#Chat) - новый чат

### PATCH /chat/`<id>`
Изменить чат
`id` - ID чата

**Тело запроса**
JSON-объект с полями:
* `name` - имя чата
* `icon_file` - название файла для иконки чата

**Ответ**
[#Structs#Chat](#Structs#Chat) - измененный чат

### GET /chat/`<id>`
Получить информацию о чате
`id` - ID чата

**Ответ**
[#Structs#Chat](#Structs#Chat) - чат

### DELETE /chat/`<id>`
Удалить чат
`id` - ID чата

**Ответ**
*Ничего*

### GET /chats
Получить список чатов, в которых состоит пользователь

**Ответ**
Список [#Structs#Chat](#Structs#Chat) - все чаты, в которых состоит пользователь

## Chat Member
### POST /chat/`<id>`/member
Добавить пользователя в чат
`id` - ID чата

**Тело запроса**
JSON-объект с полями:
* `user_id` - ID пользователя для добавления

**Ответ**
[#Structs#ChatMember](#Structs#ChatMember) - новый пользователь

### DELETE /chat/`<chat_id>`/member/`<user_id>`
Исключить из чата пользователя
`chat_id` - ID чата
`user_id` - ID пользователя

**Ответ**
*Ничего*

### GET /chat/`<chat_id>`/member/`<user_id>`
Получить информаию о пользователе в чате
`chat_id` - ID чата
`user_id` - ID пользователя

**Ответ**
[#Structs#ChatMember](#Structs#ChatMember) - информация о пользователе

### GET /chat/`<chat_id>`/members
Получить информацию о всех пользователях в чате
`chat_id` - ID чата

**Ответ**
Список [#Structs#ChatMember](#Structs#ChatMember) - все пользователи чата

### PATCH /chat/`<chat_id>`/member/`<user_id>`
Изменить права пользователя в чате
`chat_id` - ID чата
`user_id` - ID пользователя

**Тело запроса**
JSON-объект с полями:
* `can_write`: bool - может ли пользователь писать в чат
* `can_add_members`: bool - может ли пользователь добавлять других пользователей в чат
* `can_kick_members`: bool - может ли пользователь исключать участников из чата

**Ответ**
[#Structs#ChatMember](#Structs#ChatMember) - измененный пользователь

## Chat Messages
---
### POST /chat/`<chat_id>`/message
Отправить сообщение в чат
`chat_id` - ID чата

**Тело запроса**
JSON-объект с полями:
* `content`: string - текст сообщения
* `files`: Список строк - список с именами сообщений

**Ответ**
[#Structs#ChatMessage](#Structs#ChatMessage) - новое сообщение

### PATCH /chat/`<chat_id>`/message/`<message_id>`
Изменить сообщение в чате
`chat_id` - ID чата
`message_id` - ID сообщения

**Тело запроса**
JSON-объект с полями:
* `content`: string - текст сообщения
* `files`: Список строк - список с именами сообщений

**Ответ**
[#Structs#ChatMessage](#Structs#ChatMessage) - измененное сообщение

### DELETE /chat/`<chat_id>`/message/`<message_id>`
Отправить сообщение в чат
`chat_id` - ID чата
`message_id` - ID сообщения

**Ответ**
*Ничего*

### GET /chat/`<chat_id>`/messages
`chat_id` - ID чата

**Параметры URL**
`limit` - лимит сообщений для выдачи
`offset` - сдвиг сообщений в выдаче

**Ответ**
Список [#Structs#ChatMessage](#Structs#ChatMessage) - список сообщений из чата

## Events
---
### GET /events
Получить список непрочитанных событий

**Ответ**
Список [#Structs#Event](#Structs#Event) - список событий

### DELETE /events
Прочитать события

**Тело запроса**
JSON-объект с такими полями:
* `ids`: список чисел - ID событий, которые нужно прочитать

**Ответ**
JSON-объект с полями:
* `read_event_ids`: список чисел - ID прочитанных событий

## Files
---
### POST /files
Загрузить файл на сервер

**Тело запроса**: `multipart/form-data` файлы

**Ответ**
Массив с именами загруженных файлов

### GET /files/`<filename>`
Получить файл с сервера
`filename` - имя файла

# Events
## Types
### NewMessage
Название: `new_message`

Вызывается при получении нового сообщения (Отправитель сообщения так же получает это событие)

Содержание:
объект [#ChatMessage](#ChatMessage) - полученное сообщение

### MemberAdded
Название: `member_added

Вызывается при добавлении участника в чат
`
Содержание:
объект [#ChatMember](#ChatMember) - данные о добавленном участнике чата

### MemberKicked
Название: `member_kicked`

Вызывается при удалении участника из чата

Содержание:
Оъект с полями:
* `user`: [#User](#User) - пользователь, которого удалили
* `chat`: [#Chat](#Chat) - чат, из которого удалили 

## Payload
JSON структура, содержащая в себе информацию о каком-то событии

Поля:
* `type`: string - название [типа](#Types) события.
* `data`: object - содержание [типа события](#Types)

# Structs
Далее описаны JSON структуры, используемые в API.

Поля идут в формате:
`название_поля`: тип_поля - Пояснение к полю

## User
Пользователь.

Поля:
* `id`: integer - ID пользователя
* `username`: string - имя пользователя
* `icon_file`: string | null - имя [файла](#Files) иконки
* `created_at`: integer - Unix Timestamp создания пользователя
* `is_bot`: bool - Является ли пользователь ботом

## Chat
Чат.

Поля:
* `id`: integer - ID чата
* `name`: string - имя чата
* `icon_file`: string | null - имя [файла](#Files) иконки
* `created_at`: integer - Unix Timestamp создания чата
* `owner`: [#User](#User) - владелец чата

## ChatMemberPermissions
Права учатника чата

Поля:
* `can_write`: bool - Может писать в чат
* `can_add_members`: bool - Может добавлять участников чата
* `can_kick_members`: bool - Может удалять участников чата

## ChatMember
Участник чата

Поля:
* `id`: integer - ID участника чата (Не равно ID пользователя!)

* `user`: [#User](#User) - аккаунт участника
* `chat`: [#Chat](#Chat) - чат, в котором участник состоит
* `permissions`: [#ChatMemberPermissions](#ChatMemberPermissions) - права участника

## ChatMessage
Сообщение в чате

Поля:
* `id`: integer - ID сообщения
* `content`: string - Содержание сообщения
* `files`: array (string) - Массив с названиями прикрепленных файлов
* `sender`: [#User](#User) - отправитель сообщения
* `chat`: [#Chat](#Chat) - чат, в котором находится это сообщение

## Event
Событие

Поля:
* `id`: integer - ID события
* `user`: [#User](#User) - пользователь, получивший это событие
* `payload`: [#Payload](#Payload) - содержание события

## Error
Ошибка, возвращаемая каким-либо endpoint'ом.

Поля:
* `code`: integer - Код ошибки. Используются коды статуса HTTP.
* `message`: string - Пояснение ошибки.
