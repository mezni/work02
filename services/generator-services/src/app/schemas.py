from pydantic import BaseModel

class Request(BaseModel):
    interval_start: str
    interval_mins: int
    trx_count: int
