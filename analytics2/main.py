from services import Servers

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
