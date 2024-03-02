from typing import Optional

from sqlmodel import Field, SQLModel, create_engine


class Hero(SQLModel, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)
    name: str
    secret_name: str
    age: Optional[int] = None


DATABASE_URL = "postgresql://expenses:passw0rd@172.18.0.2/expenses"

engine = create_engine(DATABASE_URL)

SQLModel.metadata.create_all(engine)
