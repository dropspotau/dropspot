mod download;
mod file;
mod integration;
mod upload;
mod user;
mod utils;

#[cfg(feature = "web")]
mod web;

pub use download::{handle_file_download, handle_file_request_download};
pub use file::{handle_delete_file, handle_get_file, handle_list_files, handle_update_file};
pub use integration::{
    handle_get_integration_by_slug, handle_get_integrations, handle_upsert_integration,
};
pub use upload::{handle_file_request_upload, handle_file_upload, handle_preview_upload};
pub use user::{handle_create_user, handle_login, handle_refresh_tokens};

#[cfg(feature = "web")]
pub use web::file::handle_files;
#[cfg(feature = "web")]
pub use web::header::handle_header;
#[cfg(feature = "web")]
pub use web::index::handle_index;
#[cfg(feature = "web")]
pub use web::onboarding::handle_record_onboarding;
#[cfg(feature = "web")]
pub use web::settings::{handle_settings, handle_update_settings, handle_update_user};
