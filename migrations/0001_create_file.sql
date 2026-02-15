create extension if not exists "uuid-ossp";


create table file (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    path varchar(1028) not null,
    size bigint not null,
    created_at timestamptz not null,
    expires_at timestamptz not null,
    max_downloads int not null,
    has_uploaded boolean not null default false
);

-- 1-1 table recording how a file was uploaded
create table upload (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references file on delete cascade not null unique,
    created_at timestamptz not null,
    expires_at timestamptz not null,
    upload_started_at timestamptz,
    upload_finished_at timestamptz,
    has_uploaded boolean not null generated always as (upload_finished_at is not null) stored
);

create table download (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references file on delete cascade not null,
    created_at timestamptz not null,
    expires_at timestamptz not null
);
