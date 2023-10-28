from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from contextlib import asynccontextmanager
import datetime, random
from service import generate_cache, generate_subscribers, generate_ips, generate_events

app = FastAPI()


class Request(BaseModel):
    interval_start: str
    interval_mins: int
    trx_count: int


servers = []
devices = []
subscribers = []
ips = []
subscriber_count = 10000
ip_count = 50000
batch_id = random.randint(10000, 99999)
subscriber_prefix = "201" + str(batch_id)


@asynccontextmanager
async def lifespan(application: FastAPI):
    print ('init')
    cached_servers = await generate_cache("servers.csv")
    for s in cached_servers:
        servers.append(s)
    cached_devices = await generate_cache("devices.csv")
    for d in cached_devices:
        devices.append(d)
    generated_subscribers = await generate_subscribers(
        subscriber_prefix, subscriber_count
    )
    for s in generated_subscribers:
        subscribers.append(s)
    generated_ips = await generate_ips(ip_count)
    for i in generated_ips:
        ips.append(i)
    print (subscribers)
    yield


@app.post("/api/events", status_code=200)
async def get_events(payload: Request):
    r = payload.dict()

    if 1==1:
        interval_min_ts = int(
            datetime.datetime.strptime(
                r["interval_start"], "%d/%m/%Y %H:%M:%S"
            ).timestamp()
        )

        interval_max_ts = int(
            (
                datetime.datetime.strptime(r["interval_start"], "%d/%m/%Y %H:%M:%S")
                + datetime.timedelta(minutes=r["interval_mins"])
            ).timestamp()
        )

        trx_count = int(r["trx_count"])
        events, subscribers, ips = generate_events(
            interval_min_ts, interval_max_ts, trx_count, servers, subscribers, ips
        )
        return {"events": events}
#    except:
#        raise HTTPException(status_code=400, detail="Invalid parameters")
