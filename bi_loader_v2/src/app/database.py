from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.orm import sessionmaker, declarative_base

from app import models

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2:5432/expenses"
pool_size = 5
engine = create_async_engine(DATABASE_URL, echo=True, future=True, pool_size=pool_size)

Base = declarative_base()


async def create_db():
    async with engine.begin() as conn:
        await conn.run_sync(models.Base.metadata.create_all)


async def get_session():
    async_session = sessionmaker(bind=engine, class_=AsyncSession)
    return async_session()
