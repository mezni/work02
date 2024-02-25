from fastapi import FastAPI
from app.routers import finops_router

service_name = "Finops"
app = FastAPI(title=service_name)

app.include_router(finops_router, tags=["Finops"], prefix="/api/v1/finops")
