from pydantic import BaseModel
from sqlalchemy.ext.asyncio import AsyncSession

from .database import get_db

from models import User as UserModel


class UserSchemaBase(BaseModel):
    email: str | None = None
    full_name: str | None = None


class UserSchemaCreate(UserSchemaBase):
    pass


class UserSchema(UserSchemaBase):
    id: str

    class Config:
        orm_mode = True
