import asyncio
import motor.motor_asyncio

# MongoDB connection URL
MONGO_URL = "mongodb://localhost:27017"

# Connect to MongoDB
client = motor.motor_asyncio.AsyncIOMotorClient(MONGO_URL)

# Access database
db = client["mydatabase"]

data = {
    "key1": "value1",
    "key2": "value2",
    "key3": "value3"
}

async def insert_data():
    collection = db["mycollection"]
    await collection.insert_one(data)

asyncio.run(insert_data())
