from typing import List
from datetime import datetime, date, time
from uuid import UUID
from pydantic import BaseModel


class DateDim(BaseModel):
    date: date


class ClientDim(BaseModel):
    client_name: str
