from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import datetime, random
from service import init, generate_events

app = FastAPI()


class Request(BaseModel):
    interval_start: str
    interval_mins: int
    trx_count: int


db_name = "events.db"
batch_id = random.randint(10000, 99999)
subscriber_prefix = "201" + str(batch_id)

subscriber_count = 1000
ip_count = 5000
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
        events = generate_events(db_name, interval_min_ts, interval_max_ts, trx_count)
        return {"events": events}

    except:
        raise HTTPException(status_code=400, detail="Invalid parameters")
