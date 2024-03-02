import databases
from sqlmodel import create_engine

DATABASE_URL = "postgresql://expenses:passw0rd@172.18.0.2/expenses"
database = databases.Database(DATABASE_URL)

# Database engine
engine = create_engine(DATABASE_URL)


async def connect_db():
    await database.connect()


async def disconnect_db():
    await database.disconnect()
