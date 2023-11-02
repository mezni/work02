from fastapi import FastAPI
from app.database import init_db
from app.routers import event_router

service_name = "Generator"


app = FastAPI(title=service_name)


@app.on_event("startup")
async def startup():
    # create db tables
    await init_db()


app.include_router(event_router, tags=["Event"], prefix="/api/v1/event")
