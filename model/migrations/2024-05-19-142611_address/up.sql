PRAGMA foreign_keys = OFF;

create table stock_new (
  id integer not null primary key autoincrement,
  title text not null, 
  kind integer not null,
  description text not null,
  quantity integer not null
);

insert into stock_new (
  id, title, kind, description, quantity
)
select 
  id,
  title,
  case kind 
    when 'BigPrint' then 0
    when 'SmallPrint' then 1
    when 'Button' then 2
    else 0
  end as kind,
  description,
  quantity
from stock;

drop table stock;
alter table stock_new rename to stock;

drop table orders;
create table orders (
  id integer not null primary key autoincrement,
  name text not null,
  email text not null,
  total integer not null,
  shipped boolean not null
);

create table addresses (
  name text not null,
  id integer not null primary key autoincrement,
  order_id integer not null references orders (id),
  number integer not null,
  street text not null,
  city text not null,
  state text not null,
  zipcode integer not null
);

create table carts_new (
  id integer not null primary key autoincrement,
  quantity integer not null,
  order_id integer not null references orders (id), 
  item_id integer not null references stock (id)
);

insert into carts_new (
  id, 
  order_id, 
  item_id, 
  quantity
) 
select 
  c.id,
  c.order_id,
  p.id as item,
  c.quantity
from carts c 
join stock p on c.item_id = p.title;

drop table carts;
alter table carts_new rename to carts;

PRAGMA foreign_keys = ON;
