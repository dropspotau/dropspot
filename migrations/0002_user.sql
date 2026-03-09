-- Add migration script here
create table users (
    id uuid primary key default uuid_generate_v4(),
    first_name text not null,
    last_name text not null,
    email text not null,
    created_at timestamptz not null default now()
);

alter table file add column uploaded_by_id uuid references users on delete set null;
