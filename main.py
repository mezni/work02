import redis
import random
import datetime

r = redis.StrictRedis(
    host="172.19.0.2",
    port=6379,
    password="eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81",
    decode_responses=True,
)


def generate_random_ip():
    return ".".join(str(random.randint(0, 255)) for _ in range(4))


def ip_provisioning(ip_count):
    min_ts = 0
    max_ts = int(datetime.datetime.strptime("31/12/3000", "%d/%m/%Y").timestamp())
    for i in range(ip_count):
        ip_hash = "ip:" + generate_random_ip()
        x = r.hset(ip_hash, 1, max_ts)
        x = r.hset(ip_hash, 2, min_ts)


def subscriber_provisioning(subscriber_count):
    subsriber_suffix = "20101234"
    min_ts = 0
    max_ts = int(datetime.datetime.strptime("31/12/3000", "%d/%m/%Y").timestamp())
    for i in range(subscriber_count):
        subsriber_id = subsriber_suffix + str(random.randint(10000, 99999))
        subscriber_hash = "subscriber:" + subsriber_id
        x = r.hset(subscriber_hash, 1, max_ts)
        x = r.hset(subscriber_hash, 2, min_ts)


def get_keys(prefix):
    items_list = []
    cursor = "0"
    while cursor != 0:
        cursor, keys = r.scan(cursor=cursor, count=1000000)
        items = [k for k in keys if k.startswith(prefix)]
        items_list = items_list + items
    return items_list


ip_provisioning(5)
subscriber_provisioning(5)

ips = get_keys("ip:")
subscribers = get_keys("subscriber:")


interval_min_ts = int(
    datetime.datetime.strptime("24/10/2023 21:00:00", "%d/%m/%Y %H:%M:%S").timestamp()
)
interval_max_ts = int(
    datetime.datetime.strptime("24/10/2023 21:05:00", "%d/%m/%Y %H:%M:%S").timestamp()
)
trx_end = random.randint(interval_min_ts, interval_max_ts)
trx_duration = random.randint(0, 3600)
trx_start = trx_end - trx_duration

subscriber_hash = subscribers[random.randint(0, len(subscribers))]
s = r.hgetall(subscriber_hash)
if trx_start > int(s["2"]):
    x = r.hset(subscriber_hash, 1, trx_start)
    x = r.hset(subscriber_hash, 2, trx_end)

ip_hash = ips[random.randint(0, len(ips))]
i = r.hgetall(ip_hash)
if trx_start > int(i["2"]):
    x = r.hset(ip_hash, 1, trx_start)
    x = r.hset(ip_hash, 2, trx_end)
    print(r.hgetall(ip_hash))
