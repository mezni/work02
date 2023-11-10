import random


class Server:
    def __init__(self, server_dict) -> None:
        for key in server_dict:
            setattr(self, key, server_dict[key])


class Servers:
    def __init__(self) -> None:
        self.servers = []

    def load(self, servers_list) -> None:
        for s in servers_list:
            sl = s.split(",")
            for i in range(int(sl[-1])):
                val = sl[:-1] + [self.generate_server_ip()]
                so = Server(dict(zip(keys, val)))
                self.servers.append(so)

    def generate_server_ip(self) -> str:
        """generate server ip"""
        first_octet = random.randint(0, 254)
        if first_octet % 2 == 0:
            return (
                str(first_octet)
                + "."
                + ".".join(str(random.randint(0, 255)) for _ in range(3))
            )
        return (
            str(first_octet + 1)
            + "."
            + ".".join(str(random.randint(0, 255)) for _ in range(3))
        )


servers_list = ["Facebook,TCP,https,facebook.com,Web,social media,5"]
keys = [
    "app_name",
    "protocol",
    "app_protocol",
    "domain",
    "content_type",
    "category",
    "server_ip",
]

servers = Servers()
servers.load(servers_list)
