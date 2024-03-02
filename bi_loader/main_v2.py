import asyncio
from typing import Optional, List

from sqlalchemy.ext.asyncio import create_async_engine
from sqlalchemy.orm import sessionmaker
from sqlmodel import Field, Relationship, SQLModel, UniqueConstraint
from sqlmodel.ext.asyncio.session import AsyncSession
from sqlmodel import Session

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2/expenses"

POOL_SIZE = 10
engine = create_async_engine(
    DATABASE_URL, echo=True, future=True, pool_size=max(5, POOL_SIZE)
)

SessionLocal = sessionmaker(
    autocommit=False,
    autoflush=False,
    bind=engine,
    class_=AsyncSession,
    expire_on_commit=False,
)


async def init_db() -> None:
    async with engine.begin() as conn:
        await conn.run_sync(SQLModel.metadata.create_all)


class ProviderBase(SQLModel):
    name: str
    code: str


class Provider(ProviderBase, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)


class ProviderCreate(ProviderBase):
    pass


async def get_session():
    async with Session.begin():
        yield Session()


async def main():
    await init_db()
    async with SessionLocal() as session:
        p = {"name": "us-east-1", "code": ""}
        db_provider = ProviderCreate(**p)
        session.add(db_provider)
        session.commit()
        session.refresh(db_provider)
        print(db_provider)


asyncio.run(main())
