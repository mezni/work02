import datetime, uuid
import logging
from fastapi import APIRouter, HTTPException, status
from app import schemas, repositories, database


logger = logging.getLogger(__name__)

health_router = APIRouter()
event_router = APIRouter()


@health_router.get("/", status_code=status.HTTP_200_OK)
async def health():
    return {"status": "OK"}


@event_router.post("/", status_code=status.HTTP_200_OK)
async def get_events(payload: schemas.Request):
    logger.info("start request")
    r = payload.dict()
    try:
        interval_min_ts = int(
            datetime.datetime.strptime(
                r["interval_start"], "%d/%m/%Y %H:%M:%S"
            ).timestamp()
        )
    except:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="interval_start format should be d/m/Y H:M:S",
        )
    try:
        interval_mins = int(r["interval_mins"])
    except:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="interval_mins should be number",
        )

    try:
        trx_count = int(r["trx_count"])
    except:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST, detail="trx_count should be number"
        )

    interval_max_ts = int(
        (
            datetime.datetime.strptime(r["interval_start"], "%d/%m/%Y %H:%M:%S")
            + datetime.timedelta(minutes=r["interval_mins"])
        ).timestamp()
    )
    client_ips = await database.load_data_from_db("ips")
    subscribers = await database.load_data_from_db("subscribers")
    servers = await database.load_data_from_db("servers")
    locations = await database.load_data_from_db("locations")
    events, servers = await repositories.generate_events(
        interval_min_ts,
        interval_max_ts,
        trx_count,
        servers,
        subscribers,
        client_ips,
        locations,
    )
    await database.delete_table("servers")
    await database.load_data_to_db("servers", servers)

    response = {"Records": events}
    logger.info("end request")
    return response
