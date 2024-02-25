import asyncpg


async def get_connection():
    conn = await asyncpg.connect(
        user="finops", password="password123", database="finops-db", host="172.18.0.2"
    )
    try:
        yield conn
    finally:
        await conn.close()
