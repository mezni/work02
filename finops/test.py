import json


class FinopsConfig:
    def __init__(self, data):
        self.name = ""
        self.age = ""
        self.city = ""

    def __repr__(self):
        return f"Person(name='{self.name}', age={self.age}, city='{self.city}')"


json_data = '{"name": "Alice", "age": 25, "city": "New York"}'

data_dict = json.loads(json_data)

person_instance = FinopsConfig(data_dict)
