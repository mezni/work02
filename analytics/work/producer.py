import os, time, json, requests
from datetime import datetime, timedelta
from confluent_kafka import Producer


def get_events(events_start_date, interval_mins, trx_count):
    headers = {"Content-type": "application/json", "Accept": "text/plain"}
    data = {
        "interval_start": events_start_date,
        "interval_mins": interval_mins,
        "trx_count": trx_count,
    }

    r = requests.post(url, data=json.dumps(data), headers=headers)
    status = r.status_code
    result = r.json()
    if status == 200:
        return result["Records"]
    else:
        return []


def acked(err, msg):
    if err is not None:
        print("Failed to deliver message: %s: %s" % (str(msg), str(err)))


#    else:
#        print("Message produced: %s" % (str(msg)))


def generate_next_date(events_start_date, interval_mins, interval_multiplier):
    next_start_date_time = datetime.strptime(
        events_start_date, "%d/%m/%Y %H:%M:%S"
    ) + timedelta(minutes=interval_mins * interval_multiplier)
    next_start_date = next_start_date_time.strftime("%d/%m/%Y %H:%M:%S")
    return next_start_date


url = "http://172.18.0.100:8000/api/v1/event/"

# conf = {"bootstrap.servers": "kafka1:19092,kafka2:19093,kafka3:19094"}
conf = {"bootstrap.servers": "172.18.0.3:9092,172.18.0.4:9093,172.18.0.5:9094"}

topic = "events"

producer = Producer(**conf)

events_start_date = "19/11/2023 19:00:00"
interval_mins = 5
trx_count = 1000

f = open("lock.lck", "w")
f.close()

i = 0
while os.path.exists("lock.lck"):
    next_start_date = generate_next_date(events_start_date, interval_mins, i)
    for j in range(50):
        events = get_events(next_start_date, interval_mins, trx_count)
        for event in events:
            producer.produce(topic, json.dumps(event).encode("utf-8"), callback=acked)
        producer.poll(1)
        time.sleep(1)
    i = i + 1
