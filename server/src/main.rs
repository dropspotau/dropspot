mod auth;
mod cli;
mod db;
mod handlers;
mod middleware;
mod state;
mod storage;
mod types;
mod watch;

use clap::{Parser, Subcommand};
use uuid::Uuid;

use crate::cli::{
    auth::{handle_create_user, handle_login},
    file::{handle_download, handle_get_file, handle_list_files, handle_upload},
    server::{handle_run_server, handle_watch},
};

#[derive(Parser)]
#[command(name = "dropspot")]
#[command(about = "A simple file sharing CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum FileCommands {
    #[command(about = "Upload a file")]
    Upload { file: String },
    #[command(about = "Download a file")]
    Download {
        id: Uuid,
        key: String,
        nonce: String,
    },
    #[command(about = "List files")]
    List,
    #[command(about = "Retrieve a file")]
    Get { id: Uuid },
}

#[derive(Subcommand)]
enum ServerCommands {
    #[command(about = "Watch for files")]
    Watch,
    #[command(about = "Run the server")]
    Run,
}

#[derive(Subcommand)]
enum AuthCommands {
    #[command(about = "Log into DropSpot")]
    Login,
    #[command(about = "Create a user")]
    Create,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    File(FileCommands),

    #[command(subcommand)]
    Server(ServerCommands),

    #[command(subcommand)]
    Auth(AuthCommands),
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::File(file_commands) => match file_commands {
            FileCommands::Upload { file: file_name } => handle_upload(file_name).await,
            FileCommands::Download { id, key, nonce } => handle_download(id, key, nonce).await,
            FileCommands::List {} => handle_list_files().await,
            FileCommands::Get { id } => handle_get_file(id).await,
        },
        Commands::Server(server_commands) => match server_commands {
            ServerCommands::Watch {} => handle_watch().await,
            ServerCommands::Run => handle_run_server().await,
        },
        Commands::Auth(auth_commands) => match auth_commands {
            AuthCommands::Login {} => handle_login().await,
            AuthCommands::Create {} => handle_create_user().await,
        },
    }
}
