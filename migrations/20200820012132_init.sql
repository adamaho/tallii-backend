-- Invite Codes
create table invite_codes (
    id varchar(10) primary key
);

insert into invite_codes (id) values ('aho');

-- Users
create table users (
    username varchar(100) not null unique,
    avatar text,
    email text not null unique,
    password text not null,
    invite_code varchar not null unique references invite_codes(id),
    taunt text,
    verified boolean default false,
    created_at timestamp not null default current_timestamp,
    primary key (username)
);

-- Friends
create table friends (
    username integer not null references users(username),
    friend_username integer not null references users(username),
    state text not null, -- active, blocked
    created_at timestamp not null default current_timestamp,
    primary key (username, friend_username)
);

-- Events
create table events (
    event_id serial primary key,
    name text not null,
    description text,
    creator_username integer not null references users(username),
    created_at timestamp not null default current_timestamp
);

-- Event Members
create table events_members (
    member_id serial primary key,
    event_id integer not null references events(event_id) on delete cascade,
    username integer not null references users(username),
    status text not null default 'pending', -- pending, declined, accepted
    created_at timestamp not null default current_timestamp
);

-- Teams
create table events_teams (
    team_id serial primary key,
    event_id integer not null references events(event_id) on delete cascade,
    name text not null,
    score integer not null default 0,
    winner boolean not null default false,
    created_at timestamp not null default current_timestamp
);

-- Team Players
create table events_teams_members (
    team_id integer not null references events_teams(team_id) on delete cascade,
    member_id integer not null references events_members(member_id) on delete cascade,
    created_at timestamp not null default current_timestamp
);



