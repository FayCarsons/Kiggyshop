create table users (
  id integer primary key autoincrement,
  ip text not null,
  user_agent text,
  device text not null,
  time datetime default CURRENT_TIMESTAMP,
  country text not null,
  state text not null,
  city text
);
