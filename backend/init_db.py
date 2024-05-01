import json
import sqlite3
import os

database_url = os.getenv("DATABASE_PATH")
if not database_url:
    raise ValueError("DATABASE_URL not present in env")

conn = sqlite3.connect(database_url)
cursor = conn.cursor()

try:
    with open("stock.json") as stock:
        data = json.load(stock)

    with open("./migrations/2023-10-29-042456_init/up.sql") as query:
        cursor.execute(query.read())

    for item in data:
        cursor.execute(
            """INSERT INTO stock (title, kind, description, quantity)
                VALUES ($1, $2, $3, $4);""",
            (item["title"], item["kind"], item["description"], item["quantity"]),
        )
    conn.commit()
except Exception as e:
    print(f"An error occured in database initialization: {e}")
    conn.rollback()
finally:
    conn.close()
