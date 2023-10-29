import requests
import json
import time
from datetime import datetime, timedelta
from confluent_kafka import Producer

headers = {"Content-type": "application/json", "Accept": "text/plain"}
url = "http://172.18.0.5:8000/api/events"


data = {"interval_start": "29/10/2023 17:00:00", "interval_mins": 5, "trx_count": 5}

conf = {"bootstrap.servers": "172.18.0.3:9092"}
producer = Producer(conf)


r = requests.post(url, data=json.dumps(data), headers=headers)
status = r.status_code
result = r.json()

print(result)
