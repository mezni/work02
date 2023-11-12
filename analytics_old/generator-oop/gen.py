import uuid, random


class Event:
    def __init__(self, interval_min_ts, interval_max_ts) -> None:
        self.interval_min_ts = interval_min_ts
        self.interval_max_ts = interval_max_ts
        self.generate_event_info()
        self.generate_trx_info()

    def generate_event_info(self) -> None:
        self.timestamp = self.interval_max_ts
        self.event_id = uuid.uuid4()
        self.probe_id = probe_id
        self.correlation_id = (random.randint(1000000000, 9990000000),)
        self.location_id = locations[random.randint(0, len(locations) - 1)]

    def generate_trx_info(self) -> None:
        self.trx_end = random.randint(self.interval_min_ts, self.interval_max_ts)
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

        self.trx_start = self.trx_end - trx_duration
        self.trx_duration = trx_duration
        self.bytes_in = bytes_in
        self.bytes_out = bytes_out
        self.bytes_in_lost = bytes_in_lost
        self.bytes_out_lost = bytes_out_lost
        self.rtt_client = rtt_client
        self.rtt_server = rtt_server

    def __str__(self):
        return f"Event({self.event_id})"

    def __repr__(self):
        return f"Event({self.event_id}, {self.trx_end})"


def generate_locations(locations_number):
    locations = []
    for i in range(locations_number):
        l = str(uuid.uuid4()).split("-")[-1]
        locations.append(l)
    return locations


locations = generate_locations(5)

probe_id = uuid.uuid4()

event = Event(100, 110)
print(repr(event))
