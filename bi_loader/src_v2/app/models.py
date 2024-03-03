from sqlalchemy import Column, Integer, String, ForeignKey, select
from sqlalchemy.exc import IntegrityError, NoResultFound
from sqlalchemy.ext.asyncio import AsyncSession

from sqlalchemy.orm import declarative_base

Base = declarative_base()


class Provider(Base):
    __tablename__ = "providers"
    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    code = Column(String(60), unique=True, nullable=False, index=True)
    name = Column(String(255))

    @classmethod
    async def create(cls, db: AsyncSession, **kwargs):
        transaction = cls(**kwargs)
        db.add(transaction)
        await db.commit()
        await db.refresh(transaction)
        return transaction


class Region(Base):
    __tablename__ = "regions"
    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    code = Column(String(60), unique=True, nullable=False, index=True)
    name = Column(String(255))
    provider_id = Column(Integer, ForeignKey("providers.id"), index=True)
