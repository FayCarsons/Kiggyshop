-- Your SQL goes here
CREATE TABLE IF NOT EXISTS stock (
        id INTEGER PRIMARY KEY NOT NULL,
        title TEXT NOT NULL UNIQUE,
        kind TEXT NOT NULL,
        description TEXT NOT NULL,
        quantity INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS orders (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL, 
    street TEXT NOT NULL,
    zipcode INTEGER NOT NULL,
    fulfilled BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS carts (
    id INTEGER PRIMARY KEY NOT NULL,
    order_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id)
);