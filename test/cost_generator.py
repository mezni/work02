columns = [
    "periode",
    "client",
    "cloud",
    "compte",
    "service",
    "cout",
    "devise",
    "estimation",
]
for i in range(10):
    periode = "2023-12-06"
    client = "client1"
    cloud = "aws"
    compte = "323625553814"
    service = "AWS Lambda"
    cout = 0
    devise = "USD"
    estimation = "False"
    line = (
        periode
        + ","
        + client
        + ","
        + cloud
        + ","
        + compte
        + ","
        + service
        + ","
        + str(cout)
        + ","
        + devise
        + ","
        + estimation
    )
    print(line)
