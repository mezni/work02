from kombu import Connection, Exchange, Queue, Consumer

# amqp://username:password@hostname:port/virtualhost
rabbit_url = "amqp://dali:dali@172.18.10.2:5672/"

conn = Connection(rabbit_url)
channel = conn.channel()
exchange = Exchange("example-exchange", type="direct")
queue = Queue(name="example-queue", exchange=exchange, routing_key="BOB")


def process_message(body, message):
    print("The body is {}".format(body))
    message.ack()


with Consumer(conn, queues=queue, callbacks=[process_message], accept=["text/plain"]):
    conn.drain_events(timeout=2)
