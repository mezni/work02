import requests
import json
import pandas as pd
import matplotlib.pyplot as plt

headers = {"Content-type": "application/json", "Accept": "text/plain"}
url = "http://127.0.0.1:8000/api/v1/event/"

data = {"interval_start": "04/11/2023 19:00:00", "interval_mins": 5, "trx_count": 5000}

events = []
for i in range(5):
    r = requests.post(url, data=json.dumps(data), headers=headers)
    status = r.status_code
    result = r.json()
    events = events + result["Records"]

df = pd.DataFrame(events)
# print(df.info())
# print(df.groupby(["serverIPAddress"]).agg({"eventID": "count"}))
print(df.groupby(["subscriberID"]).agg({"eventID": "count"}))

# df.plot()
