from typing import List
from datetime import datetime, date, time
from uuid import UUID
from pydantic import BaseModel


class Holiday(BaseModel):
    holiday_date: str
    holiday_desc: str


class DateDim(BaseModel):
    start_date: str
    end_date: str
