import json
import sqlite3

database_url = "data.sqlite"

conn = sqlite3.connect(database_url)
cursor = conn.cursor()

try:
    with open("stock.json") as stock:
        data = json.load(stock)

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
