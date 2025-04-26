create extension citext;
create domain email as citext
check ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

create table subscriptions (
    id uuid primary key default uuid_generate_v7(),
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    email email not null unique,
    name text not null
);

create trigger update_updated_at_subscriptions before update on subscriptions for each row execute procedure update_updated_at();
