-- Invite Codes
create table invite_codes (
  id varchar(10) primary key
);

-- Users
create table users (
  user_id serial primary key,
  avatar text,
  name varchar(40) not null,
  email text not null unique,
  password text not null,
  invite_code varchar not null unique references invite_codes(id),
  username varchar(40) not null unique,
  taunt text,
  created_at timestamp not null default current_timestamp
);

-- Friends
create table friends (
  user_id integer not null references users(user_id),
  friend_id integer not null references users(user_id),
  friend_status text not null,
  created_at timestamp not null default current_timestamp,
  primary key (user_id, friend_id)
);


-- Groups
create table groups (
  group_id serial primary key,
  name text not null,
  description text,
  avatar text,
  created_at timestamp not null default current_timestamp
);

-- Group Users
create table group_users (
  group_id not null references groups(group_id),
  user_id not null references users(user_id),
  type varchar(20),
  created_at timestamp not null default current_timestamp
  primary key (group_id, user_id)
);