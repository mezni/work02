CREATE KEYSPACE IF NOT EXISTS events
  WITH REPLICATION = { 
   'class' : 'SimpleStrategy', 
   'replication_factor' : 1 
  };

CREATE TABLE IF NOT EXISTS events.events (
  store_key text, 
  event_ts int,
  PRIMARY KEY (store_key, event_ts)
) WITH CLUSTERING ORDER BY (event_ts ASC);