import asyncio
from models.postgres.models import User, Event

async def main():
    # user = await User.new("bluered", "aboba", "aboba@tochka.sry")
    user = await User.get_by_id(1)
    print(user.compare_password('aboba'))
    bot = await User.new_bot('abobka_bot1', user)
    print(bot.is_bot())
    creator = await bot.creator()
    if not creator is None:
        print(creator.id(), creator.username())
    else:
        print("creator is none")

asyncio.run(main())