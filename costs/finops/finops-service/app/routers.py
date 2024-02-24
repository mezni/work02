from fastapi import APIRouter, status, Depends
from app.database import get_connection
from app.schemas import ClientDim

from datetime import datetime

finops_router = APIRouter()


@finops_router.post("/client_dim", status_code=status.HTTP_201_CREATED)
async def create_client_dim(payload: ClientDim, conn=Depends(get_connection)):
    await conn.execute(
        "INSERT INTO client_dim (client_name) VALUES ($1)",
        payload.client_name,
    )
    return {"message": "Item created successfully"}
