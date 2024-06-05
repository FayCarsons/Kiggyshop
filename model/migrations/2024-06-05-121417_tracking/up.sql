create table temp_orders (
  id integer not null primary key autoincrement,
  name text not null,
  email text not null,
  total integer not null,
  shipped boolean not null default 0 check (shipped in (0, 1)),
  tracking_number text
);

insert into temp_orders (
  id, name, email, total, shipped
)
select 
  id,
  name,
  email,
  total,
  shipped 
from orders;

drop table orders;
alter table temp_orders
   rename to orders;
