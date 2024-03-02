import asyncio
from database import engine
from sqlalchemy.ext.asyncio import async_sessionmaker
from pydantic import BaseModel


class ProviderCreate(BaseModel):
    code: str
    name: str


async def main():
    session = async_sessionmaker(bind=engine, expire_on_commit=False)
    p = ProviderCreate(code="us-east-1", name="US east")
    session.add(p)
    await session.commit()


asyncio.run(main())
