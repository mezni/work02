import sqlite3, csv

conn = sqlite3.connect('events.db')
c = conn.cursor()
c.execute('CREATE TABLE if not exists servers (domain text, app_name text, protocol text, app_protocol text, content_type text, server_ip_address text)')
c.execute('CREATE TABLE if not exists servers (device_type text)')
c.execute('CREATE TABLE if not exists ips (ip text, min_ts int, max_ts int)')
c.execute('CREATE TABLE if not exists subscribers (subscriber text, min_ts int, max_ts int)')

def generate_cache(file_name):
    """generate cache"""
    fp = open("../data/" + file_name, "r")
    reader = csv.DictReader(fp)
    cache = list()
    for dict in reader:
        cache.append(dict)
    return cache

servers = generate_cache('servers.csv')
table = 'servers'
cursor = conn.cursor()
for s in servers:
    placeholders = ', '.join(['?'] * len(s))
    columns = ', '.join(s.keys())
    sql = "INSERT INTO %s ( %s ) VALUES ( %s )" % (table, columns, placeholders)
    cursor.execute(sql, list(s.values()))
conn.commit()