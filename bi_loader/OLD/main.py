import asyncio
from database import database, connect_db, disconnect_db
from sqlmodel import Session
from models import Provider, ProviderCreate


async def main():
    await connect_db()
    provider = Provider(name="xx", code="yy")
    await create_provider(get_session(), provider)


async def get_session():
    async with Session.begin():
        yield Session()


async def create_provider(session: Session, provider: ProviderCreate):
    db_provider = Provider.from_orm(provider)


asyncio.run(main())
