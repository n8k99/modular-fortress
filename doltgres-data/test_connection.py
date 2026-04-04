#!/usr/bin/env python3
import psycopg2

try:
    conn = psycopg2.connect(
        host="localhost",
        port=5432,
        user="postgres",
        password="password",
        database="postgres"
    )
    cur = conn.cursor()
    cur.execute("SELECT version();")
    version = cur.fetchone()
    print(f"✓ Connected successfully!")
    print(f"Version: {version[0]}")

    # Create our database and user
    cur.execute("CREATE DATABASE master_chronicle;")
    print("✓ Created master_chronicle database")

    cur.execute("CREATE USER nebulab_user WITH PASSWORD 'nebulab_dev_password';")
    cur.execute("GRANT ALL PRIVILEGES ON DATABASE master_chronicle TO nebulab_user;")
    print("✓ Created nebulab_user with privileges")

    conn.commit()
    cur.close()
    conn.close()
except Exception as e:
    print(f"✗ Error: {e}")
