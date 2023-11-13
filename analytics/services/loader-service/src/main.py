import requests
import json
import pandas as pd
from datetime import datetime
from cassandra.cluster import Cluster
from cassandra import ConsistencyLevel
from cassandra.query import BatchStatement


headers = {"Content-type": "application/json", "Accept": "text/plain"}
url = "http://127.0.0.1:8000/api/v1/event/"


data = {"interval_start": "09/11/2023 18:00:00", "interval_mins": 5, "trx_count": 5}

events = []
for i in range(1):
    r = requests.post(url, data=json.dumps(data), headers=headers)
    status = r.status_code
    result = r.json()
    events = events + result["Records"]

df = pd.DataFrame(events)
df["store_key"] = (
    "s#"
    + df["subscriberID"]
    + "#"
    #    + datetime.fromtimestamp(int(df["transactionEnd"])).strftime("%Y/%d%m%H")
)
df["event_ts"] = df["transactionEnd"]

cassandra_cluster = Cluster(["172.18.0.2"])
session = cassandra_cluster.connect("events")
prepared_query = session.prepare("INSERT INTO events(store_key, event_ts) VALUES (?,?)")
for item in df:
    print(item.store_key, item.event_ts)
    session.execute(prepared_query, (item.store_key, item.event_ts))
