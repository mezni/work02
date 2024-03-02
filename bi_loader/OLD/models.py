from typing import Optional
from sqlmodel import SQLModel, Field


class ProviderBase(SQLModel):
    name: str
    code: str


class Provider(ProviderBase, table=True):
    id: Optional[int] = Field(default=None, primary_key=True)


class ProviderCreate(ProviderBase):
    pass
