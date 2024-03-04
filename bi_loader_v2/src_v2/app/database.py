from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.orm import sessionmaker, declarative_base

from app import models

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2:5432/expenses"
engine = create_async_engine(DATABASE_URL, echo=True, future=True)

Base = declarative_base()

SessionLocal = sessionmaker(
    bind=engine,
    autocommit=False,
    autoflush=False,
    expire_on_commit=False,
    class_=AsyncSession,
)


async def init_db():
    async with engine.begin() as conn:
        await conn.run_sync(models.Base.metadata.create_all)
        # await conn.run_sync(SQLModel.metadata.drop_all)


async def get_session():
    try:
        session = SessionLocal
        yield SessionLocal
    except Exception as e:
        await session.rollback()
        raise e
    finally:
        await session.close()
