import asyncio
import motor.motor_asyncio

# MongoDB connection URL with authentication
host="172.18.0.2"
port="27017"
db_name="costs"
username="costs"
password="costs"


MONGO_URL = f"mongodb://{username}:{password}@{host}:{port}/{db_name}"
print (MONGO_URL)
client = motor.motor_asyncio.AsyncIOMotorClient(MONGO_URL)
db = client.get_database()


async def insert_data():
    collection = db["mycollection"]
    await collection.insert_one({"key1": "value1"})

asyncio.run(insert_data())

