import requests
import json
import pandas as pd

headers = {"Content-type": "application/json", "Accept": "text/plain"}
url = "http://172.18.0.2:8000/api/v1/event/"


data = {"interval_start": "09/11/2023 18:00:00", "interval_mins": 5, "trx_count": 5}

events = []
for i in range(1000):
    r = requests.post(url, data=json.dumps(data), headers=headers)
    status = r.status_code
    result = r.json()
    events = events + result["events"]
df = pd.DataFrame(events)
df.to_csv("events_20231109180000.csv")
