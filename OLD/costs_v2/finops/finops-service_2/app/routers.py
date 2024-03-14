from fastapi import APIRouter, status, Depends
from app.database import get_connection
import app.schemas as schemas
import app.repositories as repositories

from datetime import datetime

finops_router = APIRouter()


@finops_router.post("/date", status_code=status.HTTP_201_CREATED)
async def create_date(payload: schemas.DateDim, conn=Depends(get_connection)):
    result = await repositories.create_dim_date(payload, conn)
    return result
