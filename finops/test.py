import uuid
from datetime import datetime


class ContextManager:
    def __init__(self, config, last_state, prev_state) -> None:
        self.context_id = uuid.uuid4()
        self.start_time = datetime.now()
        self.end_time = None
        self.status = "success"
        self.message = ""
        self.config = config
        self.last_state = last_state
        self.prev_state = prev_state

    def get_context(self) -> dict:
        context = {"params": {"file_name": ""}}
        return context

    def get_curr_state(self) -> dict:
        state = {}
        return state

    def get_prev_state(self) -> dict:
        state = {}
        return state


config = {}
last_state = {}
prev_state = {}
context = ContextManager(config, last_state, prev_state)
print(context.get_context())
