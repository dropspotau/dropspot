# DropSpot
A self-hostable server for timed-out and limited file uploads.

## Contains:
* A server for running a web server which handles file uploads, downloads and user authentication
* CLI tool for interacting with the server from the command line
* Rust library to use from your own program
* JavaScript library to use from websites (the same Rust library, compiled to WebAssembly!)

## Integrations
* Local storage
* Google Cloud Storage
* AWS S3 (coming soon&trade;!)

## Setup
The setup assumes you have a PostgreSQL database running with a url provided in the `DROPSPOT_DATABASE_URL` environment variable.

Requirements:

### Server:
* `cargo`
* `sqlx`
* Postgres >=18.0
* The ability to at write to your OS's temporary directory

Setup:
```
# Migrate the database
./scripts/migrate-database.sh

# Run the server
dropspot server run

# Delete any files after they've been expired
dropspot server watch
```

### CLI:
```
# Runs a local DropSpot server
dropspot server run

# Watches and deletes any files which have expired
dropspot file watch

# Create a DropSpot user (optional)
dropspot auth create

# Log into said user
dropspot auth login

# Upload a file
dropspot file upload <file_name>

# Download a file
dropspot file download <file_id>

# Retrieve a file's details (requires authentication)
dropspot file get <file_id>

# Retrieve all files' details (requires authentication)
dropspot file list
```

If running through `cargo`, simply replace the `dropspot` command with `cargo run --package dropspot-server`

## Crates
### Core
The `dropspot-core` crate provides all the user-facing functionality needed to interact with the server

### Server
The `dropspot-server` crate provides the server logic, database integration and CLI tooling required to run and interact with DropSpot

## Features
| Name | Description | Default |
| --------------- | --------------- | --------------- |
| `client` | Allows the `auth` and `file` commands to be run in the CLI | ✅ |
| `server` | Enables the `server` commands to be run in the CLI | ✅ |
| `web` | Enables web endpoints in the server | ✅ |



## Running the local setup
```
bacon run-server
bacon build-web
```


### Building WebAssembly
Run the `./scripts/build-wasm.sh` script :)

### Building the web (assuming the WASM package has been built)
```
cd web
pnpm install
pnpm build
```

### Migrating the database (required if compiling with SQLX online)
```
./scripts/migrate-database.sh
```
