-- Add migration script here
create table users (
    id uuid primary key default uuid_generate_v4(),
    first_name text not null,
    last_name text not null,
    email text not null,
    created_at timestamptz not null default now()
);

alter table file add column uploaded_by_id uuid references users on delete set null;

create table organisation (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    created_at timestamptz not null default now()
);

create table member (
    id uuid primary key default uuid_generate_v4(),
    organisation_id uuid not null references organisation on delete cascade,
    user_id uuid not null references users on delete cascade,
    created_at timestamptz not null default now(),

    unique (organisation_id, user_id)
);
