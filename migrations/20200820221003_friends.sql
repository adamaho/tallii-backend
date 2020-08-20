create table friends (
  user_id integer not null references users(user_id),
  friend_id integer not null references users(user_id),
  friend_status text not null,
  created_at timestamp not null default current_timestamp,
  primary key (user_id, friend_id)
);