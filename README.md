# Bitum
**Bitum** - мессенджер с поддержкой ботов. Выполнен в качестве учебного проекта в [Яндекс Лицей](https://lyceum.yandex.ru/).

Backend сайта выполнен на **Flask**, в качестве базы данных используется **PostgreSQL**, ORM - **peewee**.

Frontend построен на языке **rust**, используется фреймворк **Yew**.
Выполнен в стиле SinglePage сайта.

# Установка
## Нужные программы
Для установки вам потребуются следующие программы:
* [Git](https://git-scm.com) - git, что бы склонировать этот репозиторий (Можно и без него)
* [python3 и pip3](https://python.org) - Интерпретатор и пакетный менеджер питона. Желательно питон версии 3.10
* [PostgreSQL](https://www.postgresql.org/) - сервер базы данных.
* [Rust](https://rust-lang.org) - язык программирования , на котором написан фронтэнд
* [trunk](https://trunkrs.dev/#install) - инструмент для компилирования WebAssembly бинарников на языке **Rust**.

## Начало установки
Для начала нужно установить и настроить **PostgreSQL** базу данных (как это делать не расскажу, скажу лишь что нам нужна одна база данных, и пользователь, имеющий полный доступ к этой БД)

После этого клонируем этот репозиторий командой:
```bash
git clone https://github.com/radyshenkya/bitum.git
```

Теперь нужно скомпилировать фронтэнд.
Для этого переходим в папку `frontend/bitum-frontend`, оттуда вызываем команду:
```bash
trunk build
```

## .ENV
После установки нужно настроить файл `.env` в корне проекта. Изначально его нет, его нужно создать самому.

**Никому не показывайте этот файл, там хранятся приватные данные**

Этот файл имеет такие переменные:
```env
DB_NAME=
DB_USER=
DB_PASSWORD=
DB_HOST=
DB_PORT=

JWT_SECRET=
```

Все эти поля должны быть заполнены

Значения полей:
* `DB_NAME` - имя базы данных
* `DB_USER` - имя пользователя от БД
* `DB_PASSWORD` - пароль от пользователя БД
* `DB_HOST` - домен/IP адресс базы данных. Без указания протокола.
* `DB_PORT` - порт, на котором стоит БД (по умолчанию 5432 для PostgreSQL).
* `JWT_SECRET` - секретная фраза, используемая для шифровки токенов. **Это должен быть длинный набор случайных символов**

Пример содержания файла **.env**:
```env
DB_NAME=test_db
DB_USER=test_db_user
DB_PASSWORD=super_secret_db_pass
DB_HOST=example.com
DB_PORT=5432
JWT_SECRET=$uP3r-s3c537_K3y
```

## Установка пакетов
Установка пакетов происходит командой
```bash
pip install -r requirements.txt
```

## Запуск сайта

Прописываем команду
```
gunicorn --bind 0.0.0.0:8000 wsgi:app
```

По необходимости меняем айпи сервера/порт.

Готово!
---
# Тех. часть
1. **[requirements.txt](https://github.com/radyshenkya/bitum/blob/main/requirements.txt)**
2. **bootstrap** - подключается в файле [index.html](https://github.com/radyshenkya/bitum/blob/main/frontend/bitum-frontend/index.html) (и используется в последствии)
3. **Шаблоны** - не используются
4. **ORM-модели** - Описаны в файле [database.py](https://github.com/radyshenkya/bitum/blob/main/models/postgres/database.py). Используется библиотека Peewee.
5. **Регистрация и авторизация** - для этого используются токены, и API методы `POST /api/user` (для регистрации / создания пользователя), `POST /api/user/token` (для создания токена пользователя по его имени и паролю).
6. **Загрузка и использование файлов** - API метод `POST /api/files` - для загрузки файлов. В ответ возвращаются имена загруженных файлов на сервере. Получить эти файлы можно из эндпоинта `GET /files/<FILE_NAME>`
7. **API: REST** - все методы API описаны в файле [api/endpoints.py](https://github.com/radyshenkya/bitum/blob/main/api/endpoints.py).
8. **Хранение данных** - используется база данных PostgreSQL. Подключение к БД, как и модели ORM можно просмотреть в файле [database.py](https://github.com/radyshenkya/bitum/blob/main/models/postgres/database.py)
9. **Хостинг** - сайт работает на хостинге [Glitch](https://glitch.com). [Ссылка на сам проект](https://bitumsite.glitch.me)
