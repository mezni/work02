from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import datetime, random, uuid
from app.service import init, generate_events
from app.config import settings


class Request(BaseModel):
    interval_start: str
    interval_mins: int
    trx_count: int


app = FastAPI()

instance_id = str(uuid.uuid4())

db_name = settings.DB_NAME
subscriber_count = settings.SUBSCRIBER_COUNT
ip_count = settings.IP_COUNT

batch_id = random.randint(10000, 99999)
subscriber_prefix = "201" + str(batch_id)


init(db_name, subscriber_prefix, subscriber_count, ip_count)


@app.post("/api/events", status_code=200)
async def get_events(payload: Request):
    r = payload.dict()

    try:
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
        events = generate_events(instance_id, db_name, interval_min_ts, interval_max_ts, trx_count)
        return {"events": events}

    except:
        raise HTTPException(status_code=400, detail="Invalid parameters")
