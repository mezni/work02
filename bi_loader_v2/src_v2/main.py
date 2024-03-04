import asyncio
from app.database import init_db, get_session


async def main():
    await init_db()
    session = await get_session()


asyncio.run(main())
