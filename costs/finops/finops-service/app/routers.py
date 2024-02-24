from fastapi import APIRouter, status, Depends
from app.database import get_connection
from app.schemas import DateDim

from datetime import datetime

finops_router = APIRouter()


@finops_router.post("/date_dimension", status_code=status.HTTP_201_CREATED)
async def create_date_dimension(payload: DateDim, conn=Depends(get_connection)):
    await conn.execute(
        "INSERT INTO date_dimension (date_value) VALUES ($1)",
        datetime.strptime(payload.date, "%Y-%m-%d"),
    )
    return {"message": "Item created successfully"}
