import uuid
from datetime import datetime

__author__ = "Mohamed Ali MEZNI"
__version__ = "2023-12-24"


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
        self.execution = self.set_execution()
        self.variables = self.set_variables()
        self.params = self.set_params()

    def set_execution(self) -> dict:
        self.end_time = datetime.now()
        execution = {
            "context_id": str(self.context_id),
            "start_time": self.start_time.strftime("%d-%m-%Y %H:%M:%S"),
            "end_time": self.end_time.strftime("%d-%m-%Y %H:%M:%S"),
            "status": self.status,
            "message": self.message,
        }
        return execution

    def set_variables(self) -> dict:
        return {}

    def set_params(self) -> dict:
        params = {
            "start_date": "",
            "end_date": "",
            "granularity": "DAILY",
            "dimensions": ["LINKED_ACCOUNT", "SERVICE"],
            "metrics": ["BlendedCost"],
            "filters": "",
        }
        return params

    def get_context(self) -> dict:
        context = {"execution": self.execution, "params": self.params}
        return context

    def get_curr_state(self) -> dict:
        state = {}
        return state

    def get_prev_state(self) -> dict:
        state = {}
        return state


config = {"client_name": "client1", "client_code": "client1"}
last_state = {}
prev_state = {}
context = ContextManager(config, last_state, prev_state)
print(context.get_context())
