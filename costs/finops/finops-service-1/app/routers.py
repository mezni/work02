from fastapi import APIRouter, status, Depends
from app.database import get_connection
from app.schemas import ClientDim
import app.repositories as repositories

from datetime import datetime

finops_router = APIRouter()


@finops_router.post("/client", status_code=status.HTTP_201_CREATED)
async def create_client(payload: ClientDim, conn=Depends(get_connection)):
    result = await repositories.create_client(payload, conn)
    return result


"""
@finops_router.post("/client", status_code=status.HTTP_201_CREATED)
async def create_client(payload: ClientDim, conn=Depends(get_connection)):
    await conn.execute(
        "INSERT INTO client_dim (client_name) VALUES ($1)",
        payload.client_name,
    )
    return {"message": "Item created successfully"}
"""

#    result = await repositories.create_moteur(
#        session=session, payload=payload.dict())
