import asyncio
from sqlalchemy.ext.asyncio import create_async_engine, AsyncSession
from sqlalchemy.orm import sessionmaker
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy import Column, Integer, String

# SQLAlchemy Base
Base = declarative_base()


# SQLAlchemy Model
class User(Base):
    __tablename__ = "users"

    id = Column(Integer, primary_key=True, index=True)
    username = Column(String, unique=True, index=True)
    email = Column(String, unique=True, index=True)


# Database connection URL
DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2:5432/expenses"

# Create an async engine
engine = create_async_engine(DATABASE_URL, echo=True, future=True)

# Create async session
async_session = sessionmaker(bind=engine, class_=AsyncSession, expire_on_commit=False)


# Function to add user asynchronously
async def add_user(username: str, email: str):
    async with async_session() as session:
        user = User(username=username, email=email)
        session.add(user)
        await session.commit()
        await session.refresh(user)
        return user


# Asynchronous entry point
async def main():
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)

    user = await add_user("john_doe1", "john1@example.com")
    print("Added user:", user)


# Run the async entry point
asyncio.run(main())
