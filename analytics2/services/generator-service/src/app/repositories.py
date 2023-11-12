import random, uuid
from app.data import file_type, app_name, probe_id


async def generate_server_ip() -> str:
    """generate server ip"""
    first_octet = random.randint(0, 254)
    if first_octet % 2 == 0:
        return (
            str(first_octet)
            + "."
            + ".".join(str(random.randint(0, 255)) for _ in range(3))
        )
    return (
        str(first_octet + 1)
        + "."
        + ".".join(str(random.randint(0, 255)) for _ in range(3))
    )


async def generate_client_ip() -> str:
    """generate client ip"""
    first_octet = random.randint(0, 254)
    if first_octet % 2 == 0:
        return (
            str(first_octet + 1)
            + "."
            + ".".join(str(random.randint(0, 255)) for _ in range(3))
        )
    return (
        str(first_octet) + "." + ".".join(str(random.randint(0, 255)) for _ in range(3))
    )


async def generate_servers(servers_list: list, server_keys: list) -> list:
    """generate servers"""
    servers = []
    for s in servers_list:
        sl = s.split(",")
        for i in range(int(sl[-1])):
            val = sl[:-1] + [await generate_server_ip()]
            so = dict(zip(server_keys, val))
            servers.append(so)

    return servers


async def generate_locations(location_count: int) -> list:
    """generate locations"""
    locations = []
    for i in range(location_count):
        location_id = str(uuid.uuid4()).split("-")[-1]
        item = {"location": location_id}
        locations.append(item)
    return locations


async def generate_subscribers(subscriber_prefix, subscriber_count):
    """generate subscribers"""
    subscribers = []
    for i in range(subscriber_count):
        subscriber = subscriber_prefix + str(random.randint(100000, 999999))
        item = {"subscriber": subscriber, "min_ts": 0, "max_ts": 0}
        subscribers.append(item)
    return subscribers


async def generate_ips(ip_count):
    """generate ips"""
    ips = []
    for i in range(ip_count):
        ip = await generate_client_ip()
        item = {"ip": ip, "min_ts": 0, "max_ts": 0}
        ips.append(item)
    return ips


async def generate_app_info():
    """generate server info"""
    info = {
        "file_type": file_type,
        "app_name": app_name,
        "app_instance": random.randint(1000, 9999),
        "app_id": random.randint(10000, 99999),
        "probe_id": probe_id,
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
