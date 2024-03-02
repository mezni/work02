from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.orm import sessionmaker, declarative_base

from app import models

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2:5432/expenses"
engine = create_async_engine(DATABASE_URL, echo=True, future=True)

Base = declarative_base()


async def init_db():
    async with engine.begin() as conn:
        await conn.run_sync(models.Base.metadata.create_all)
        # await conn.run_sync(SQLModel.metadata.drop_all)


#        await conn.run_sync(SQLModel.metadata.create_all)


async def get_session() -> AsyncSession:
    async_session = sessionmaker(
        bind=engine, class_=AsyncSession, expire_on_commit=False
    )
    async with async_session() as session:
        yield session


async_session = sessionmaker(bind=engine, class_=AsyncSession)


async def get_async_session():
    async with async_session() as session:
        yield session
