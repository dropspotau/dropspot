create extension if not exists "uuid-ossp";

create schema if not exists dropspot;

create table dropspot.organisation (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    created_at timestamptz not null default now()
);

create table dropspot.users (
    id uuid primary key default uuid_generate_v4(),
    email text not null,
    first_name text not null,
    last_name text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    updated_by_id uuid references dropspot.users (id) on delete set null,
    organisation_id uuid not null references dropspot.organisation (id) on delete cascade,
    is_admin boolean not null,

    unique (organisation_id, id)
);

create table dropspot.password (
    user_id uuid primary key references dropspot.users (id) on delete cascade,
    password text not null,
    updated_at timestamptz
);

create type storage as enum ('local', 's3', 'gcs');

create table dropspot.file (
    id uuid primary key default uuid_generate_v4(),
    name text not null,
    size bigint not null,
    created_at timestamptz not null,
    created_by_id uuid references dropspot.users (id) on delete set null,
    expires_at timestamptz not null,
    max_downloads int not null,
    has_uploaded boolean not null default false,
    storage storage not null,
    has_expired boolean not null default false,
    updated_at timestamptz,
    updated_by_id uuid references dropspot.users (id) on delete set null
);

-- 1-1 table recording how a file was uploaded. A file can only be uploaded by one person
create table dropspot.upload (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references dropspot.file (id) on delete cascade not null unique,
    created_at timestamptz not null,
    expires_at timestamptz not null,
    upload_started_at timestamptz,
    upload_finished_at timestamptz,
    upload_ip inet not null,
    has_uploaded boolean not null generated always as (upload_finished_at is not null) stored
);

-- But a file can be downloaded by multiple people
create table dropspot.download (
    id uuid primary key default uuid_generate_v4(),
    file_id uuid references dropspot.file (id) on delete cascade not null,
    created_at timestamptz not null,
    created_by_id uuid references dropspot.users (id) on delete set null,
    download_ip inet not null,
    expires_at timestamptz not null
);

create table dropspot.integration (
    id uuid primary key default uuid_generate_v4(),
    slug storage not null,
    organisation_id uuid references dropspot.organisation (id) on delete cascade not null,
    is_active boolean not null,
    data jsonb not null,
    updated_at timestamptz,
    updated_by_id uuid references dropspot.users (id) on delete set null,

    unique(slug, organisation_id) -- One integration per type per organisation
);

create table dropspot.settings (
    id uuid primary key default uuid_generate_v4(),
    organisation_id uuid references dropspot.organisation (id) on delete cascade not null unique,
    default_file_expiry_minutes int not null check (default_file_expiry_minutes > 0),
    default_download_limit int not null check (default_download_limit > 0),
    allow_external_uploads boolean not null,
    allow_external_downloads boolean not null,
    max_file_size_mb integer not null,
    updated_at timestamptz,
    updated_by_id uuid references dropspot.users (id) on delete set null
);

-- A record of each user's onboarding completion
create table dropspot.onboarding (
    id uuid primary key default uuid_generate_v4(),
    user_id uuid references dropspot.users (id) on delete cascade not null unique,
    completed_at timestamptz
);


--
-- Indexes
--

create index idx_user_email on dropspot.users (email);
create index idx_user_updated_by_id on dropspot.users (updated_by_id);
create index idx_user_organisation_id on dropspot.users (organisation_id);
create index idx_password_user_id on dropspot.password (user_id);
create index idx_file_created_by_id on dropspot.file (created_by_id);
create index idx_file_updated_by_id on dropspot.file (updated_by_id);
create index idx_upload_file_id on dropspot.upload (file_id);
create index idx_download_file_id on dropspot.download (file_id);
create index idx_download_created_by_id on dropspot.download (created_by_id);
create index idx_integration_organisation_id on dropspot.integration (organisation_id);
create index idx_integration_updated_by_id on dropspot.integration (updated_by_id);
create index idx_settings_organisation_id on dropspot.settings (organisation_id);
create index idx_settings_updated_by_id on dropspot.settings (updated_by_id);


--
-- Data
--


-- Create an initial organisation with one local file integration
with default_organisation_id as (
    insert into dropspot.organisation (name)
    values ('Default')
    returning id
)
insert into dropspot.integration (slug, organisation_id, is_active, data)
values (
	'local'::storage,
    (select id from default_organisation_id limit 1),
	true,
	'{"folder": "files"}'::jsonb
);

-- And settings
with default_organisation_id as (
    select id
    from dropspot.organisation
    where name = 'Default'
)
insert into dropspot.settings (organisation_id, default_file_expiry_minutes, default_download_limit, allow_external_uploads, allow_external_downloads, max_file_size_mb)
values ((select id from default_organisation_id limit 1), 60, 3, true, true, 10);
