from typing import List, Optional
from sqlmodel import SQLModel, Field, create_engine, Session
import databases
from pydantic import BaseModel

# Database setup
DATABASE_URL = "postgresql://username:password@localhost/db_name"
database = databases.Database(DATABASE_URL)
metadata = SQLModel.metadata


# SQLModel Definition
class User(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    name: str
    email: str


# Pydantic Model for API
class UserCreate(BaseModel):
    name: str
    email: str


class UserUpdate(BaseModel):
    name: Optional[str] = None
    email: Optional[str] = None


# Connect to database
async def connect_db():
    await database.connect()


# Disconnect from database
async def disconnect_db():
    await database.disconnect()


# Create a new user
async def create_user(user: UserCreate):
    query = User.__table__.insert().values(name=user.name, email=user.email)
    user_id = await database.execute(query)
    return {**user.dict(), "id": user_id}


# Get a user by ID
async def get_user(user_id: int):
    query = User.__table__.select().where(User.id == user_id)
    return await database.fetch_one(query)


# Get all users
async def get_users():
    query = User.__table__.select()
    return await database.fetch_all(query)


# Update user
async def update_user(user_id: int, user_update: UserUpdate):
    query = (
        User.__table__.update()
        .where(User.id == user_id)
        .values(**user_update.dict(exclude_unset=True))
    )
    await database.execute(query)
    return {"message": "User updated successfully"}


# Delete user
async def delete_user(user_id: int):
    query = User.__table__.delete().where(User.id == user_id)
    await database.execute(query)
    return {"message": "User deleted successfully"}
