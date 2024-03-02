import asyncio
from typing import List

from sqlalchemy.ext.asyncio import create_async_engine
from sqlalchemy.orm import sessionmaker
from sqlmodel import Field, Relationship, SQLModel, UniqueConstraint
from sqlmodel.ext.asyncio.session import AsyncSession

DATABASE_URL = "postgresql+asyncpg://expenses:passw0rd@172.18.0.2/expenses"

POOL_SIZE = 10
engine = create_async_engine(
    DATABASE_URL, echo=True, future=True, pool_size=max(5, POOL_SIZE)
)


async def init_db() -> None:
    async with engine.begin() as conn:
        await conn.run_sync(SQLModel.metadata.create_all)


SessionLocal = sessionmaker(
    autocommit=False,
    autoflush=False,
    bind=engine,
    class_=AsyncSession,
    expire_on_commit=False,
)


class HouseLocationLink(SQLModel, table=True):
    house_id: int = Field(foreign_key="house.id", nullable=False, primary_key=True)
    location_id: int = Field(
        foreign_key="location.id", nullable=False, primary_key=True
    )


class Location(SQLModel, table=True):
    id: int = Field(primary_key=True)
    type: str  # country, county, municipality, district, city, area, street, etc
    name: str  # Amsterdam, Germany, My Street, etc

    houses: List["House"] = Relationship(
        back_populates="locations",
        link_model=HouseLocationLink,
    )

    __table_args__ = (UniqueConstraint("type", "name"),)


class House(SQLModel, table=True):
    id: int = Field(primary_key=True)
    color: str = Field()
    locations: List["Location"] = Relationship(
        back_populates="houses",
        link_model=HouseLocationLink,
    )
    # other fields...


data = [
    {
        "color": "red",
        "locations": [
            {"type": "country", "name": "Netherlands"},
            {"type": "municipality", "name": "Amsterdam"},
        ],
    },
    {
        "color": "green",
        "locations": [
            {"type": "country", "name": "Netherlands"},
            {"type": "municipality", "name": "Amsterdam"},
        ],
    },
]


async def add_houses(payload) -> List[House]:
    result = []
    async with SessionLocal() as session:
        for item in payload:
            locations = []
            for location in item["locations"]:
                locations.append(Location(**location))
            house = House(color=item["color"], locations=locations)
            result.append(house)
        session.add_all(result)
        await session.commit()


asyncio.run(init_db())
asyncio.run(add_houses(data))
