import csv, random, uuid, datetime


def generate_cache(file_name):
    """generate cache"""
    fp = open("data/" + file_name, "r")
    reader = csv.DictReader(fp)
    cache = list()
    for dict in reader:
        cache.append(dict)
    return cache


def generate_random_ip():
    """generate single ip"""
    return ".".join(str(random.randint(0, 255)) for _ in range(4))


def generate_subscribers(subscriber_prefix, subscriber_count):
    """generate subscribers"""
    subscribers = []
    for i in range(subscriber_count):
        subscriber = subscriber_prefix + str(random.randint(10000, 99999))
        item = {"subscriber": subscriber, "min_ts": 0, "max_ts": 0}
        subscribers.append(item)
    return subscribers


def generate_ips(ip_count):
    """generate ips"""
    ips = []
    for i in range(ip_count):
        ip = generate_random_ip()
        item = {"ip": ip, "min_ts": 0, "max_ts": 0}
        ips.append(item)
    return ips


def get_trx_ts():
    """generate trx ts"""
    trx_end = random.randint(interval_min_ts, interval_max_ts)
    if random.randint(0, 10) > 9:
        trx_duration = random.randint(0, 3600)
    elif random.randint(0, 10) > 8:
        trx_duration = random.randint(0, 900)
    else:
        trx_duration = random.randint(0, 60)
    trx_start = trx_end - trx_duration
    return trx_start, trx_end, trx_duration


def add_subscriber():
    """add subscriber"""
    trx_start, trx_end, trx_duration = get_trx_ts()
    subscriber = subscribers[random.randint(0, len(subscribers) - 1)]

    if trx_duration == 0:
        slot_start = max(trx_start, interval_max_ts)
        slot_end = slot_start
    elif subscriber["min_ts"] == 0:
        slot_start = trx_start
        slot_end = trx_end
    elif trx_end < subscriber["min_ts"]:
        slot_start = subscriber["min_ts"] - trx_duration
        slot_end = subscriber["min_ts"]
    elif trx_start > subscriber["max_ts"]:
        slot_start = subscriber["max_ts"]
        slot_end = min(subscriber["min_ts"] + trx_duration, interval_max_ts)
    elif trx_duration < interval_max_ts - subscriber["max_ts"]:
        slot_start = subscriber["max_ts"]
        slot_end = min(subscriber["min_ts"] + trx_duration, interval_max_ts)
    elif trx_duration < interval_max_ts - subscriber["max_ts"]:
        slot_start = subscriber["max_ts"]
        slot_end = min(subscriber["min_ts"] + trx_duration, interval_max_ts)
    elif trx_duration < subscriber["min_ts"] - interval_min_ts:
        slot_start = subscriber["min_ts"] - trx_duration
        slot_end = subscriber["min_ts"]
    else:
        trx_duration = 0
        slot_start = subscriber["max_ts"]
        slot_end = subscriber["max_ts"]

    trx_start = slot_start
    trx_end = trx_end

    new_subscriber = {
        "subscriber": subscriber["subscriber"],
        "min_ts": slot_start,
        "max_ts": slot_end,
    }
    return subscriber, new_subscriber, trx_start, trx_end, trx_duration


def add_ip(subscriber):
    """add ip"""
    ip = ips[random.randint(0, len(ips) - 1)]
    return ip


batch_id = random.randint(10000, 99999)
subscriber_prefix = "201" + str(batch_id)

servers = generate_cache("servers.csv")
devices = generate_cache("devices.csv")

subscribers = generate_subscribers(subscriber_prefix, 1000)
ips = generate_ips(10000)

interval_min_ts = int(
    datetime.datetime.strptime("25/10/2023 21:00:00", "%d/%m/%Y %H:%M:%S").timestamp()
)
interval_max_ts = int(
    datetime.datetime.strptime("25/10/2023 21:05:00", "%d/%m/%Y %H:%M:%S").timestamp()
)


events = []
file_type = "AllIPMessages"
app_name = "TrafficServerElement"
app_instance = random.randint(1000, 9999)
app_id = random.randint(10000, 99999)
for i in range(10000):
    random_server_id = random.randint(0, len(servers) - 1)
    bytes_in = random.randint(0, 102400)
    bytes_out = random.randint(0, 21600)
    if random.randint(0, 100) > 95:
        bytes_in_lost = min(random.randint(0, 2160), bytes_in)
        bytes_out_lost = min(random.randint(0, 2160), bytes_out)
    else:
        bytes_in_lost = 0
        bytes_out_lost = 0

    old_subscriber, new_subscriber, trx_start, trx_end, trx_duration = add_subscriber()
    subscribers.remove(old_subscriber)
    subscribers.append(new_subscriber)
    ip = add_ip(new_subscriber)
    event = {
        "Timestamp": str(interval_max_ts),
        "type": file_type,
        "appName": app_name,
        "appInstance": app_instance,
        "appID": app_id,
        "eventID": str(uuid.uuid4()),
        "correletionID": random.randint(1000000000, 9990000000),
        "TransactionStart": trx_start,
        "TransactionEnd": trx_end,
        "TransactionDuration": trx_duration,
        "ClientIPAddress": ip["ip"],
        "ClientPort": random.randint(1024, 52000),
        "ServerIPAddress": servers[random_server_id]["serverIPAddress"],
        "ServerPort": 443,
        "ipProtocol": servers[random_server_id]["protocol"],
        "bytesFromClient": bytes_out,
        "bytesToClient": bytes_in,
        "bytesFromServer": bytes_in,
        "bytesToServer": bytes_out,
        "SubscriberID": new_subscriber["subscriber"],
        "applicationProtocol": servers[random_server_id]["appProtocol"],
        "applicationName": servers[random_server_id]["appName"],
        "domain": servers[random_server_id]["domain"],
        "deviceType": devices[random.randint(0, len(devices) - 1)],
        "contentType": servers[random_server_id]["contentType"],
        "lostBytesClient": bytes_in_lost,
        "lostBytesServer": bytes_out_lost,
        "srttMsClient": random.randint(0, 500),
        "srttMsServer": random.randint(0, 500),
    }
    events.append(event)
    
