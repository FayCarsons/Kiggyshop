create table stock (
  id integer primary key,
  title text unique not null,
  kind text not null,
  description text not null,
  quantity integer not null
);

create table carts (
  id integer primary key,
  order_id integer not null,
  item_id text not null,
  quantity integer not null,
  foreign key (order_id) references orders (id)
);

create table orders (
  id integer primary key,
  name text not null,
  street text not null,
  zipcode text not null,
  fulfilled integer not null
);
