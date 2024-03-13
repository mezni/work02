from sqlalchemy import Column, Integer, BigInteger, String, ForeignKey, select
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
    async def create_all(cls, db: AsyncSession, data: list):
        result = []
        if data:
            try:
                transaction = []
                for d in data:
                    transaction.append(cls(**d))
                db.add_all(transaction)

                await db.commit()
            except Exception as e:
                await db.rollback()
                return None, str({e})

            for trx in transaction:
                await db.refresh(trx)
                result = [
                    {
                        key: getattr(obj, key)
                        for key in obj.__dict__
                        if not key.startswith("_")
                    }
                    for obj in transaction
                ]

        return result, None

    @classmethod
    async def get_all(cls, db: AsyncSession):
        result = []
        try:
            transaction = (await db.execute(select(cls))).scalars().all()
            result = [
                {
                    key: getattr(obj, key)
                    for key in obj.__dict__
                    if not key.startswith("_")
                }
                for obj in transaction
            ]
        except Exception as e:
            return None, str({e})

        return result, None
