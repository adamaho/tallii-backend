-- Invite Codes
create table invite_codes (
    id varchar(10) primary key
);

insert into invite_codes (id) values ('aho');

-- Users
create table users (
    user_id serial primary key,
    avatar text,
    email text not null unique,
    password text not null,
    invite_code varchar not null unique references invite_codes(id),
    username varchar(40) not null unique,
    taunt text,
    verified boolean default false,
    created_at timestamp not null default current_timestamp
);

-- Friends
create table friends (
    user_id integer not null references users(user_id),
    friend_id integer not null references users(user_id),
    friend_status text not null, -- pending, accepted, blocked
    created_at timestamp not null default current_timestamp,
    primary key (user_id, friend_id)
);

-- Events
create table events (
    event_id serial primary key,
    name text not null,
    description text,
    creator_user_id integer not null references users(user_id),
    created_at timestamp not null default current_timestamp
);

-- Events Participants
create table events_players (
    event_player_id serial primary key,
    event_id integer not null references events(event_id) on delete cascade,
    user_id integer not null references users(user_id),
    status text not null default 'pending', -- pending, declined, accepted
    created_at timestamp not null default current_timestamp
);

-- Events Teams
create table events_teams (
    event_team_id serial primary key,
    event_id integer not null references events(event_id) on delete cascade,
    name text not null,
    score integer not null default 0,
    winner boolean not null default false,
    created_at timestamp not null default current_timestamp
);

-- Event Team Participants
create table events_teams_players (
    event_team_id integer not null references events_teams(event_team_id) on delete cascade,
    event_player_id integer not null references events_players(event_player_id) on delete cascade,
    created_at timestamp not null default current_timestamp
);



