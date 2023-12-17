from datetime import datetime, timedelta


class ConfigManager:
    def __init__(self):
        self.start_time = datetime.now()
        self.end_time = None

    def set_end_time(self):
        self.end_time = datetime.now()


c = ConfigManager()
c.set_end_time()
print(c.start_time)
print(c.end_time)
