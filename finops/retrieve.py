import pymongo
import pandas as pd

client = pymongo.MongoClient("mongodb://172.18.0.2:27017/")
db = client["store"]
collection = db["cost"]

query = {"meta.file_id": {"$eq": "7c140726-aded-4ec2-929f-f13f64a29eb2"}}

results = collection.find(query)

for result in results:
    print(result)
