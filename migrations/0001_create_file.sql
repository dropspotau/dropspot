create extension if not exists "uuid-ossp";

create table upload (
    id uuid primary key,
    created_at timestamptz not null,
    expires_at timestamptz not null
);

create table file (
    id uuid primary key,
    name text not null,
    upload_id uuid references upload on delete cascade not null unique,
    path varchar(1028) not null,
    size bigint not null,
    created_at timestamptz not null,
    expires_at timestamptz not null
);
