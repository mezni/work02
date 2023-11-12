import logging
from fastapi import FastAPI

from prometheus_fastapi_instrumentator import Instrumentator


from app.database import init_db


service_name = "Generator"

logging.config.fileConfig("logging.conf", disable_existing_loggers=False)
logger = logging.getLogger(__name__)


app = FastAPI(title=service_name)
Instrumentator().instrument(app).expose(app)


@app.on_event("startup")
async def startup():
    logger.info("service start")
    await init_db()


@app.get("/")
async def init():
    return {"result": "OK"}
