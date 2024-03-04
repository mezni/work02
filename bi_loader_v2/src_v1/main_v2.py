import asyncio
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Column, Integer, String
from app.models import Provider
from app.database import init_db, get_session, get_async_session

# SQLAlchemy Base
Base = declarative_base()


# Database connection URL
# DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2:5432/expenses"

# Create an async engine
# engine = create_async_engine(DATABASE_URL, echo=True, future=True)

# Create async session
# async_session = sessionmaker(bind=engine, class_=AsyncSession, expire_on_commit=False)

from pydantic import BaseModel


class ProviderCreate(BaseModel):
    code: str
    name: str


from sqlalchemy.ext.asyncio import AsyncSession


# Function to add user asynchronously
async def add_provider(payload: dict):
    async with get_async_session() as session:
        async with session.begin():
            provider = Provider(**payload)
            session.add(provider)
            await session.commit()
            await session.refresh(provider)
            return provider


# Asynchronous entry point
async def main():
    #    async with engine.begin() as conn:
    #        await conn.run_sync(Base.metadata.create_all)

    #    provider = await add_provider("aws", "aws")
    #    print("Added provider:", provider)
    provider = ProviderCreate(code="azure", name="azure")
    p = await add_provider(provider.dict())
    print("Added provider:", provider)


# Run the async entry point
asyncio.run(main())
