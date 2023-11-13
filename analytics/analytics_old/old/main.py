import random


def generate_server_ip() -> str:
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


servers_list = [
    "facebook,tcp,https,facebook.com,web,social media,2",
    "facebook,tcp,https,app.facebook.com,web,social media,7",
    "twitter,tcp,https,twitter.com,web,social media,3",
    "instagram,tcp,https,instagram.com,web,social media,4",
    "tiktok,tcp,https,app.tiktok.com,web,social media,9",
    "xbox,udp,quick,app.xbox.com,web,gaming,9",
    "twitch,udp,quick,twitch.com,web,gaming,5",
    "twitch,udp,quick,app.twitch.com,web,gaming,5",
    "nintindo,udp,quick,app.nintindo.com,web,gaming,3",
    "ubisoft,udp,quick,app.ubisoft.com,web,gaming,3",
]


keys = [
    "app_name",
    "protocol",
    "app_protocol",
    "domain",
    "content_type",
    "category",
    "server_ip",
]


# servers = Servers()
# servers.load(servers_list)

for s in servers_list:
    sl = s.split(",")
    for i in range(int(sl[-1])):
        val = sl[:-1] + [generate_server_ip()]
        so = dict(zip(keys, val))
        print(so)
