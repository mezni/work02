from sqlalchemy.ext.asyncio import create_async_engine

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2/expenses"

POOL_SIZE = 10
engine = create_async_engine(
    DATABASE_URL, echo=True, future=True, pool_size=max(5, POOL_SIZE)
)
