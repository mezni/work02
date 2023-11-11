import aiosqlite


class SqliteStore:
    def __init__(self, db_name) -> None:
        self.db_name = db_name

    async def connect(self) -> None:
        self.conn = await aiosqlite.connect(self.db_name)


async def xx():
    db = SqliteStore("Test.db")
    return await db.connect()


x = await xx()
