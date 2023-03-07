import asyncio
from models.postgres.models import User, Chat, Event

async def main():
    user: User = (await User.search_users("test"))[0]
    
    for chat in await user.chats():
        print(chat.name())

        for member in await chat.members():
            print('MEMBER: ', (await member.user()).username())

        for message in await chat.messages(0, 10):
            print('MSG: ', message.content())


    # for event in await user.get_unread_events():
    #     print(event.payload())
    #     await event.mark_as_read()

    # user = await User.new("test", "test", "test@example.com")
    # chat: Chat = await Chat.new("test chat", user)
    # await chat.send_message(user, "Hello World!", ['aboba.png', 'piska.png'])

    # bot = await User.new_bot("test_bot", user)
    # await chat.add_member(bot)
    # await chat.send_message(bot, "hello world from bot!", [])

asyncio.run(main())