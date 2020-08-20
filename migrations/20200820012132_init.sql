create table invite_codes (
  id varchar(10) primary key
);

create table users (
  user_id serial primary key,
  avatar text,
  name varchar(40) not null,
  email text not null unique,
  password text not null,
  invite_code varchar not null unique references invite_codes(id),
  username varchar(40) not null unique,
  chirp text,
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp
);

create table friends (
  user_id integer not null references users(user_id),
  friend_id integer not null references users(user_id),
  friend_status text not null,
  created_at timestamp not null default current_timestamp,
  primary key (user_id, friend_id)
);