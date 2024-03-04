import asyncio
from app.database import create_db, get_session
from app.models import Provider


async def main():
    #    await create_db()
    async with await get_session() as session:
        x = {"code": "azure10", "name": "azure"}
        p, err = await Provider.create(session, **x)
        if not err:
            print(p.__dict__)
        else:
            print("No insert")

        p = await Provider.get_all(session)
        for i in p:
            print(i.__dict__)
        print(p)


asyncio.run(main())
