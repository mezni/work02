import uuid
import pymongo
import pandas as pd

data = {
    "date": ["2024-02-27", "2024-02-27", "2024-02-27"],
    "org": ["momentum", "momentum", "momentum"],
    "provider": ["aws", "aws", "aws"],
    "service": ["EC2", "S3", "Lambda"],
    "cost": [0.0001, 0.0002, 0.0003],
}

df = pd.DataFrame(data)
data_records = df.to_dict(orient="records")
file_id = str(uuid.uuid4())
store_records = []
for r in data_records:
    e = {"meta": {"file_id": file_id}, "record": r}
    store_records.append(e)

client = pymongo.MongoClient("mongodb://172.18.0.2:27017/")
db = client["store"]
collection = db["cost"]
collection.insert_many(store_records)
