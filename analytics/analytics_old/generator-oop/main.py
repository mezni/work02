import uuid
from dataclasses import dataclass, field


@dataclass
class Config:
    interval_start: str
    interval_mins: int
    trx_count: int


@dataclass
class Application:
    start: int
    end: int
    duration: int


class Event:
    def __init__(self) -> None:
        self.event_id = uuid.uuid4()

    def generate(self) -> None:
        self.xxxx = 3

    def print(self) -> str:
        print(self.xxxx)


event = Event()
event.generate()
event.print()
