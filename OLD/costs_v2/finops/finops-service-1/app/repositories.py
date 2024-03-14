from app.database import get_connection


async def create_client(payload: dict, session):
    q = await session.execute(
        "INSERT INTO client_dim (client_name) VALUES ($1)",
        payload.client_name,
    )
    return {"message": "Item created successfully"}
