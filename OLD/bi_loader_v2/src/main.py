import asyncio
from app.database import create_db, get_session
from app.models import Provider

data = [
    {"code": "aws", "name": "Amazon"},
    {"code": "azure", "name": "Microsoft"},
    {"code": "oci", "name": "Oracle"},
    {"code": "snowflake"},
]


async def main():
    await create_db()
    async with await get_session() as session:

        p, err = await Provider.create_all(session, data)
        print(p)
        print(err)


#        p, err = await Provider.get_all(session)
#        print(p)
#        print(err)


asyncio.run(main())
