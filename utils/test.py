
import aiosqlite, asyncio, time

async def test():
    db_name = "test.db"
    conn = await aiosqlite.connect(db_name)
    cursor = await conn.execute(
            "CREATE TABLE if not exists subscribers (subscriber text, min_ts int, max_ts int)"
        )
    await conn.close()

if __name__ == "__main__":
    start = time.time()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(test())
    end = time.time()
    print(f"Time: {end-start:.2f} sec")
