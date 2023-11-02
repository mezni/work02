import random, uuid
from app.data import servers_list, devices


async def generate_random_ip():
    """generate single ip"""
    return ".".join(str(random.randint(0, 255)) for _ in range(4))


async def generate_ips(ip_count):
    """generate ips"""
    ips = []
    for i in range(ip_count):
        ip = await generate_random_ip()
        item = {"ip": ip, "min_ts": 0, "max_ts": 0}
        ips.append(item)
    return ips


async def generate_subscribers(subscriber_prefix, subscriber_count):
    """generate subscribers"""
    subscribers = []
    for i in range(subscriber_count):
        subscriber = subscriber_prefix + str(random.randint(10000, 99999))
        item = {"subscriber": subscriber, "min_ts": 0, "max_ts": 0}
        subscribers.append(item)
    return subscribers


async def generate_server_info():
    """generate server info"""
    servers = []
    for sl in servers_list:
        for i in range(int(sl.split(",")[5])):
            s = {
                "domain": sl.split(",")[0],
                "app_name": sl.split(",")[1],
                "protocol": sl.split(",")[2],
                "app_protocol": sl.split(",")[3],
                "content_type": sl.split(",")[4],
                "server_ip": await generate_random_ip(),
                "server_port": 443,
            }
            servers.append(s)
    return servers


async def generate_app_info():
    """generate server info"""
    info = {
        "file_type": "AllIPMessages",
        "app_name": "TrafficServerElement",
        "app_instance": random.randint(1000, 9999),
        "app_id": random.randint(10000, 99999),
        "probe_id": str(uuid.uuid4()),
    }
    return info


async def generate_trx(interval_min_ts, interval_max_ts):
    """generate trx ts"""
    trx = {}
    trx_end = random.randint(interval_min_ts, interval_max_ts)

    rand_int = random.randint(0, 10)
    if rand_int > 9:
        trx_duration = random.randint(0, 3600)
        bytes_in = random.randint(0, 102400)
        bytes_out = random.randint(0, 51200)
        if random.randint(0, 100) > 98:
            bytes_in_lost = random.randint(0, 1024)
            bytes_out_lost = random.randint(0, 512)
        else:
            bytes_in_lost = 0
            bytes_out_lost = 0
    elif rand_int > 8:
        trx_duration = random.randint(0, 900)
        bytes_in = random.randint(0, 25600)
        bytes_out = random.randint(0, 12800)
        if random.randint(0, 100) > 98:
            bytes_in_lost = random.randint(0, 256)
            bytes_out_lost = random.randint(0, 128)
        else:
            bytes_in_lost = 0
            bytes_out_lost = 0
    else:
        trx_duration = random.randint(0, 60)
        bytes_in = random.randint(0, 3200)
        bytes_out = random.randint(0, 1600)
        if random.randint(0, 100) > 98:
            bytes_in_lost = random.randint(0, 32)
            bytes_out_lost = random.randint(0, 16)
        else:
            bytes_in_lost = 0
            bytes_out_lost = 0

    if random.randint(0, 10) > 9:
        rtt_client = random.randint(0, 1000)
        rtt_server = random.randint(0, 1000)
    else:
        rtt_client = random.randint(0, 500)
        rtt_server = random.randint(0, 500)

    trx_start = trx_end - trx_duration

    trx = {
        "trx_start": trx_start,
        "trx_end": trx_end,
        "trx_duration": trx_duration,
        "bytes_in": bytes_in,
        "bytes_out": bytes_out,
        "bytes_in_lost": bytes_in_lost,
        "bytes_out_lost": bytes_out_lost,
        "rtt_client": rtt_client,
        "rtt_server": rtt_server,
    }
    return trx


async def add_subscriber(interval_min_ts, interval_max_ts, trx_info, subscribers):
    """add subscriber"""
    trx_start = trx_info["trx_start"]
    trx_end = trx_info["trx_end"]
    trx_duration = trx_info["trx_duration"]

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


async def generate_events(
    interval_min_ts, interval_max_ts, trx_count, servers, subscribers, client_ips
):
    app_info = await generate_app_info()

    events = []
    for i in range(trx_count):
        trx_info = await generate_trx(interval_min_ts, interval_max_ts)
        (
            old_subscriber,
            new_subscriber,
            trx_start,
            trx_end,
            trx_duration,
        ) = await add_subscriber(
            interval_min_ts, interval_max_ts, trx_info, subscribers
        )
        trx_info["trx_start"] = trx_start
        trx_info["trx_end"] = trx_end
        trx_info["trx_duration"] = trx_duration

        subscribers.remove(old_subscriber)
        subscribers.append(new_subscriber)

        client_port = random.randint(1025, 65000)

        server_id = random.randint(0, len(servers) - 1)

        event = {
            "timestamp": interval_max_ts,
            "type": app_info["file_type"],
            "appName": app_info["app_name"],
            "appInstance": app_info["app_instance"],
            "appID": app_info["app_id"],
            "probeID": app_info["probe_id"],
            "eventID": str(uuid.uuid4()),
            "correletionID": random.randint(1000000000, 9990000000),
            "transactionStart": trx_info["trx_start"],
            "transactionEnd": trx_info["trx_end"],
            "transactionDuration": trx_info["trx_duration"],
            "clientIPAddress": client_ips[random.randint(0, len(client_ips) - 1)]["ip"],
            "clientPort": client_port,
            "serverIPAddress": servers[server_id]["server_ip"],
            "serverPort": servers[server_id]["server_port"],
            "ipProtocol": servers[server_id]["protocol"],
            "bytesFromClient": trx_info["bytes_out"],
            "bytesToClient": trx_info["bytes_in"],
            "bytesFromServer": trx_info["bytes_in"],
            "bytesToServer": trx_info["bytes_out"],
            "SubscriberID": "",
            "applicationProtocol": servers[server_id]["app_protocol"],
            "applicationName": servers[server_id]["app_name"],
            "domain": servers[server_id]["domain"],
            "deviceType": devices[random.randint(0, len(devices) - 1)],
            "contentType": servers[server_id]["content_type"],
            "lostBytesClient": trx_info["bytes_in_lost"],
            "lostBytesServer": trx_info["bytes_out_lost"],
            "srttMsClient": trx_info["rtt_client"],
            "srttMsServer": trx_info["rtt_server"],
        }
        events.append(event)
    return events
