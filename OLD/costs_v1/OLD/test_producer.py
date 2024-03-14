import pika

# RabbitMQ server credentials
credentials = pika.PlainCredentials("dali", "dali")

# Establish a connection to RabbitMQ server
connection = pika.BlockingConnection(
    pika.ConnectionParameters("localhost", credentials=credentials)
)
channel = connection.channel()

# Declare a queue named 'hello' to which messages will be published
channel.queue_declare(queue="hello")

# Publish a message to the 'hello' queue
channel.basic_publish(exchange="", routing_key="hello", body="Hello, RabbitMQ!")

print(" [x] Sent 'Hello, RabbitMQ!'")

# Close the connection to RabbitMQ server
connection.close()
