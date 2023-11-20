CREATE TABLE IF NOT EXISTS events.events_raw (
    bucket text PRIMARY KEY,
    "applicationName" text,
    "applicationProtocol" text,
    "bytesFromClient" bigint,
    "bytesFromServer" bigint,
    "bytesToClient" bigint,
    "bytesToServer" bigint,
    category text,
    "clientIPAddress" text,
    "clientPort" int,
    "contentType" text,
    "deviceType" text,
    domain text,
    "eventID" text,
    "ipProtocol" text,
    "locationID" text,
    "lostBytesClient" bigint,
    "lostBytesServer" bigint,
    "networkType" text,
    "probeID" text,
    "serverIPAddress" text,
    "serverPort" int,
    "srttMsClient" bigint,
    "srttMsServer" bigint,
    "subscriberID" bigint,
    "transactionDuration" bigint,
    "transactionEnd" bigint,
    "transactionStart" bigint
) WITH ID = 3cc45e70-8736-11ee-95bb-83075bd545ed
    AND additional_write_policy = '99p'
    AND bloom_filter_fp_chance = 0.01
    AND caching = {'keys': 'ALL', 'rows_per_partition': 'NONE'}
    AND cdc = false
    AND comment = ''
    AND compaction = {'class': 'org.apache.cassandra.db.compaction.SizeTieredCompactionStrategy', 'max_threshold': '32', 'min_threshold': '4'}
    AND compression = {'chunk_length_in_kb': '16', 'class': 'org.apache.cassandra.io.compress.LZ4Compressor'}
    AND memtable = 'default'
    AND crc_check_chance = 1.0
    AND default_time_to_live = 0
    AND extensions = {}
    AND gc_grace_seconds = 864000
    AND max_index_interval = 2048
    AND memtable_flush_period_in_ms = 0
    AND min_index_interval = 128
    AND read_repair = 'BLOCKING'
    AND speculative_retry = '99p';
