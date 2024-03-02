from sqlalchemy import Column, Integer, String, ForeignKey, select
from sqlalchemy.orm import declarative_base

Base = declarative_base()


class Provider(Base):
    __tablename__ = "provider"
    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    code = Column(String(60), unique=True, nullable=False, index=True)
    name = Column(String(255))


class Region(Base):
    __tablename__ = "region"
    id = Column(Integer, primary_key=True, index=True, autoincrement=True)
    code = Column(String(60), unique=True, nullable=False, index=True)
    name = Column(String(255))
    provider_id = Column(Integer, ForeignKey("provider.id"), index=True)
