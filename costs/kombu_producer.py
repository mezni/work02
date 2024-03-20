from kombu import Connection, Exchange, Producer, Queue


# amqp://username:password@hostname:port/virtualhost
rabbit_url = "amqp://dali:dali@172.18.10.2:5672/"

conn = Connection(rabbit_url)
channel = conn.channel()
exchange = Exchange("example-exchange", type="direct")
producer = Producer(exchange=exchange, channel=channel, routing_key="BOB")
queue = Queue(name="example-queue", exchange=exchange, routing_key="BOB")
queue.maybe_bind(conn)
queue.declare()
producer.publish("Hello there!")
