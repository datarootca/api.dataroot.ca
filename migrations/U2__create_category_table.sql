CREATE TABLE IF NOT EXISTS category (
    id SERIAL primary key,
    name varchar(63) not null,
    description varchar(511),
    is_active boolean default true,
    created_at timestamptz default now(),
    updated_at timestamptz default now()
);