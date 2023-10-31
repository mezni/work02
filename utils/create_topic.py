import confluent_kafka.admin, pprint

conf        = {'bootstrap.servers': '172.18.0.3:9092'}
kafka_admin = confluent_kafka.admin.AdminClient(conf)

new_topic   = confluent_kafka.admin.NewTopic("test03", 2, 1)
                  # Number-of-partitions  = 1
                  # Number-of-replicas    = 1


x=kafka_admin.create_topics([new_topic,]) # CREATE (a list(), so you can create multiple).
print(x)
#pprint.pprint(kafka_admin.list_topics().topics) # LIST
print (kafka_admin.list_topics().topics)
