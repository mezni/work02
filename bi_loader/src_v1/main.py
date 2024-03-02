from database import get_async_session
from app.models import User
from sqlalchemy import select


async def example_usage():
    async with await get_async_session() as session:
        async with session.begin():
            # Example query
            result = await session.execute(select(User))
            users = result.scalars().all()
            for user in users:
                print(user)


# Run the example usage
import asyncio

asyncio.run(example_usage())
