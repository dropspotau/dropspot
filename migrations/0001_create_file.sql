create extension if not exists "uuid-ossp";

create table upload (
    id uuid primary key default uuid_generate_v4(),
    created_at timestamptz not null,
    expires_at timestamptz not null
);

create table file (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    upload_id uuid references upload on delete cascade not null unique,
    path varchar(1028) not null,
    size bigint not null,
    created_at timestamptz not null,
    expires_at timestamptz not null
);

create table download (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references file on delete cascade not null,
    created_at timestamptz not null,
    expires_at timestamptz not null
);
