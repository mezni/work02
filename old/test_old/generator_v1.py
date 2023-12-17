import datetime, random
import redis


def generate_random_ip():
    return ".".join(str(random.randint(0, 255)) for _ in range(4))


def generate_ips(ip_count):
    ips = []
    min_ts = 0
    max_ts = int(datetime.datetime.strptime("31/12/3000", "%d/%m/%Y").timestamp())
    for i in range(ip_count):
        ip = generate_random_ip()
        item = {"ip": ip, "min_ts": max_ts, "max_ts": min_ts}
        ips.append(item)
    return ips


def load_ips(ips):
    for ip in ips:
        ip_hash = "ip:" + ip["ip"]
        x = r.hset(ip_hash, 1, ip["min_ts"])
        x = r.hset(ip_hash, 2, ip["max_ts"])


r = redis.StrictRedis(
    host="172.19.0.2",
    port=6379,
    password="eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81",
    decode_responses=True,
)


def get_keys(prefix):
    items_list = []
    cursor = "0"
    for k in r.scan_iter(prefix + "*"):
        item = {
            "ip": k.replace(prefix, ""),
            "min_ts": int(r.hget(k, 1)),
            "max_ts": int(r.hget(k, 2)),
        }
        items_list.append(item)
    return items_list


# ips=generate_ips(10)
# load_ips(ips)
ips = get_keys("ip:")

interval_min_ts = int(
    datetime.datetime.strptime("24/10/2023 21:00:00", "%d/%m/%Y %H:%M:%S").timestamp()
)
interval_max_ts = int(
    datetime.datetime.strptime("24/10/2023 21:05:00", "%d/%m/%Y %H:%M:%S").timestamp()
)

for i in range(10):
    trx_end = random.randint(interval_min_ts, interval_max_ts)
    if random.randint(0, 10) > 9:
        trx_duration = random.randint(0, 3600)
    elif random.randint(0, 10) > 8:
        trx_duration = random.randint(0, 900)
    else:
        trx_duration = random.randint(0, 60)

    trx_start = trx_end - trx_duration

    ip = ips[random.randint(0, len(ips) - 1)]
    if trx_start > ip["max_ts"] or trx_end < ip["min_ts"]:
        item = {"ip": ip["ip"], "min_ts": trx_start, "max_ts": trx_end}
        ips.remove(ip)
        ips.append(item)
        ip_hash = "ip:" + ip["ip"]
        x = r.hset(ip_hash, 1, trx_start)
        x = r.hset(ip_hash, 2, trx_end)
    else:
        print(ip)
        print(trx_start, trx_end)
