from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.orm import sessionmaker

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2:5432/expenses"

engine = create_async_engine(DATABASE_URL)

async_session = sessionmaker(bind=engine, class_=AsyncSession)


async def get_async_session():
    return async_session()
