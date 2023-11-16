import aiosqlite
import logging
from app.config import settings
from app.repositories import (
    generate_servers,
    generate_locations,
    generate_subscribers,
    generate_ips,
)
from app.data import subscriber_prefix, servers_list, server_keys


logger = logging.getLogger(__name__)

db_name = settings.DB_NAME
subscriber_count = settings.SUBSCRIBER_COUNT
ip_count = settings.IP_COUNT
location_count = settings.LOCATION_COUNT


async def create_db():
    conn = await aiosqlite.connect(db_name)
    cursor = await conn.execute(
        "CREATE TABLE if not exists servers (app_name text, protocol text, app_protocol text, domain text, content_type text, category text, server_ip text)"
    )

    cursor = await conn.execute("CREATE TABLE if not exists locations (location text)")

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
            if table_name == "locations":
                key = "location"
                res = {key: row[0]}
                result.append(res)
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
                    "app_name": row[0],
                    "protocol": row[1],
                    "app_protocol": row[2],
                    "domain": row[3],
                    "content_type": row[4],
                    "category": row[5],
                    "server_ip": row[6],
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
    logger.info("load servers")
    servers = await generate_servers(servers_list, server_keys)
    await load_data_to_db("servers", servers)
    logger.info("load locations")
    locations = await generate_locations(location_count)
    await load_data_to_db("locations", locations)
    logger.info("load subscribers")
    subscribers = await generate_subscribers(subscriber_prefix, subscriber_count)
    await load_data_to_db("subscribers", subscribers)
    logger.info("load ips")
    ips = await generate_ips(ip_count)
    await load_data_to_db("ips", ips)
    logger.info("db init end")
