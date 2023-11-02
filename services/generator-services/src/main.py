import logging
from fastapi import FastAPI

from prometheus_fastapi_instrumentator import Instrumentator

from app.database import init_db
from app.routers import event_router

service_name = "Generator"

logging.config.fileConfig("logging.conf", disable_existing_loggers=False)
logger = logging.getLogger(__name__)

app = FastAPI(title=service_name)

Instrumentator().instrument(app).expose(app)


@app.on_event("startup")
async def startup():
    logger.info("service start")
    await init_db()


app.include_router(event_router, tags=["Event"], prefix="/api/v1/event")
