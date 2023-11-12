import random, uuid

batch_id = random.randint(1000, 9999)
subscriber_prefix = "201" + str(batch_id)
probe_id = uuid.uuid4()
file_type = "AllIPMessages"
app_name = "TrafficServerElement"

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
    "skype,udp,quick,app.skype.com,quick,voip,10",
    "whatsapp,udp,quick,app.whatsapp.com,quick,voip,10",
    "signal,udp,quick,app.signal.com,quick,voip,5",
    "youtube,udp,quick,app.youtube.com,quick,video,9",
    "amazon,udp,quick,app.amazon.com,quick,video,8",
    "netflix,udp,quick,app.netflix.com,quick,video,8",
    "microsoft update,tcp,https,app.microsoft.com,web,content downstream,10",
    "ubuntu,tcp,https,ubuntuone.com,web,content downstream,7",
    "dns,udp,dns,dns.google.com,text,content downstream,1",
    "dns,udp,dns,dns.aws.com,text,content downstream,1",
    "mqtt,udp,mqtt,-,text,content downstream,1",
    "mysql,udp,mysql,-,text,content downstream,1",
    "ntp,udp,ntp,-,text,content downstream,1",
    "mariadb,udp,mariadb,-,text,content downstream,1",
    "oracle,udp,oracle,-,text,content downstream,1",
    "mongodb,udp,mongodb,-,text,content downstream,1",
    "tor,tcp,https,tor.com,web,content upstream,3",
    "edonky,tcp,https,edonky.com,web,content upstream,3",
    "sherazad,tcp,https,sherazad.com,web,content upstream,1",
    "mail,udp,icmp,mail.google.com,web,content upstream,2",
    "mail,udp,icmp,mail.yahoo.com,web,content upstream,2",
    "mail,udp,icmp,mail.outlook.com,web,content upstream,2",
    "-,tcp,https,-,web,content upstream,12",
    "-,tcp,http,-,web,-,5",
    "-,tcp,https,-,web,-,10",
    "-,udp,quick,-,web,-,5",
    "-,udp,-,-,web,-,5",
]


server_keys = [
    "app_name",
    "protocol",
    "app_protocol",
    "domain",
    "content_type",
    "category",
    "server_ip",
]
