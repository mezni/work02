import asyncpg


async def get_connection():
    conn = await asyncpg.connect(
        user="finops", password="password123", database="finops-db", host="172.18.0.2"
    )
    try:
        yield conn
    finally:
        await conn.close()


class BiWorker:
    def __init__(self) -> None:
        pass

    def read_file():
        pass

    def read_table():
        conn = await get_connection()
        result = await conn.fetch(f"select * from holidays")


b = BiWorker()
b.read_table()
