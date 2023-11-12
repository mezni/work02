import aiosqlite
import random
import logging
from app.config import settings
from app.repositories import generate_ips, generate_subscribers, generate_server_info

logger = logging.getLogger(__name__)

db_name = settings.DB_NAME
subscriber_count = settings.SUBSCRIBER_COUNT
ip_count = settings.IP_COUNT


batch_id = random.randint(10000, 99999)
subscriber_prefix = "201" + str(batch_id)


async def create_db():
    conn = await aiosqlite.connect(db_name)
    cursor = await conn.execute(
        "CREATE TABLE if not exists servers (domain text, app_name text, protocol text, app_protocol text,content_type text, server_ip text,server_port text)"
    )

    cursor = await conn.execute(
        "CREATE TABLE if not exists subscribers (subscriber text, min_ts int, max_ts int)"
    )

    cursor = await conn.execute(
        "CREATE TABLE if not exists ips (ip text, min_ts int, max_ts int)"
    )

    await conn.close()


async def load_data_to_db(table_name, data):
    """Load data"""
    conn = await aiosqlite.connect(db_name)
    for d in data:
        placeholders = ", ".join(["?"] * len(d))
        columns = ", ".join(d.keys())
        sql = "INSERT INTO %s ( %s ) VALUES ( %s )" % (
            table_name,
            columns,
            placeholders,
        )
        await conn.execute(sql, list(d.values()))
    await conn.commit()
    await conn.close()


async def load_data_from_db(table_name):
    """Load data"""
    result = []
    conn = await aiosqlite.connect(db_name)
    async with conn.execute("SELECT * FROM " + table_name) as cursor:
        async for row in cursor:
            if table_name == "ips":
                key = "ip"
                res = {key: row[0], "min_ts": row[1], "max_ts": row[2]}
                result.append(res)
            if table_name == "subscribers":
                key = "subscriber"
                res = {key: row[0], "min_ts": row[1], "max_ts": row[2]}
                result.append(res)
            if table_name == "servers":
                res = {
                    "domain": row[0],
                    "app_name": row[1],
                    "protocol": row[2],
                    "app_protocol": row[3],
                    "content_type": row[4],
                    "server_ip": row[5],
                    "server_port": row[6],
                }
                result.append(res)
    await conn.close()
    return result


async def delete_table(table_name):
    conn = await aiosqlite.connect(db_name)
    cursor = await conn.execute("DELETE FROM " + table_name)
    await conn.commit()
    await conn.close()


async def init_db():
    logger.info("db init start")
    await create_db()
    logger.info("generate ips")
    client_ips = await generate_ips(ip_count)
    logger.info("load ips")
    await load_data_to_db("ips", client_ips)
    logger.info("generate subscribers")
    subscribers = await generate_subscribers(subscriber_prefix, subscriber_count)
    logger.info("load subscribers")
    await load_data_to_db("subscribers", subscribers)
    logger.info("generate servers")
    servers = await generate_server_info()
    logger.info("load servers")
    await load_data_to_db("servers", servers)
    logger.info("db init end")
