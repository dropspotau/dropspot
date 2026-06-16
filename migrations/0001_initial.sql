create extension if not exists "uuid-ossp";

create table users (
    id uuid primary key default uuid_generate_v4(),
    email text not null,
    password text not null,
    first_name text not null,
    last_name text not null,
    created_at timestamptz not null default now()
);

create table organisation (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    created_at timestamptz not null default now()
);

create table member (
    id uuid primary key default uuid_generate_v4(),
    organisation_id uuid not null references organisation (id) on delete cascade,
    user_id uuid not null references users on delete cascade,
    created_at timestamptz not null default now(),
    is_admin boolean not null,

    unique (organisation_id, user_id)
);

create type storage as enum ('local', 's3', 'gcs');

create table file (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    path varchar(1028) not null,
    size bigint not null,
    created_at timestamptz not null,
    created_by_id uuid references users (id) on delete set null,
    expires_at timestamptz not null,
    max_downloads int not null,
    has_uploaded boolean not null default false,
    storage storage not null
);


-- 1-1 table recording how a file was uploaded. A file can only be uploaded by one person
create table upload (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references file (id) on delete cascade not null unique,
    created_at timestamptz not null,
    expires_at timestamptz not null,
    upload_started_at timestamptz,
    upload_finished_at timestamptz,
    upload_ip inet not null,
    has_uploaded boolean not null generated always as (upload_finished_at is not null) stored
);


-- But a file can be downloaded by multiple people
create table download (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references file (id) on delete cascade not null,
    created_at timestamptz not null,
    created_by_id uuid references users (id) on delete set null,
    download_ip inet not null,
    expires_at timestamptz not null
);

create table integration (
    id uuid primary key default uuid_generate_v4(),
    slug storage not null,
    organisation_id uuid references organisation (id) on delete cascade not null,
    is_active boolean not null,
    data jsonb not null,

    unique(slug, organisation_id) -- One integration per type per organisation
);

create table settings (
    id uuid primary key default uuid_generate_v4(),
    organisation_id uuid references organisation (id) on delete cascade not null unique,
    default_file_expiry_minutes int not null check (default_file_expiry_minutes > 0),
    default_download_limit int not null check (default_download_limit > 0)
);

-- A record of each user's onboarding completion
create table onboarding (
    id uuid primary key default uuid_generate_v4(),
    user_id uuid references users (id) on delete cascade not null unique,
    completed_at timestamptz
);


--
-- Indexes
--

create index idx_member_organisation_id on member (organisation_id);
create index idx_member_user_id on member (user_id);
create index idx_file_created_by_id on file (created_by_id);
create index idx_upload_file_id on upload (file_id);
create index idx_download_file_id on download (file_id);
create index idx_download_created_by_id on download (created_by_id);
create index idx_settings_organisation_id on settings (organisation_id);


--
-- Data
--


-- Create an initial organisation with one local file integration
with default_organisation_id as (
    insert into organisation (name)
    values ('Default')
    returning id
)
insert into integration (slug, organisation_id, is_active, data)
values (
	'local'::storage,
    (select id from default_organisation_id limit 1),
	true,
	'{"folder": "files"}'::jsonb
);

-- And settings
with default_organisation_id as (
    select id
    from organisation
    where name = 'Default'
)
insert into settings (organisation_id, default_file_expiry_minutes, default_download_limit)
values ((select id from default_organisation_id limit 1), 60, 3);
