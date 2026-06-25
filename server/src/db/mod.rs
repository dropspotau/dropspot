mod connect;
mod download;
mod error;
mod file;
mod integration;
mod member;
mod onboarding;
mod organisation;
mod settings;
mod types;
mod upload;
mod user;

pub use connect::connect;
pub use download::{Download, create_download, get_download_by_id, get_downloads_for_file};
pub use file::{
    File, create_file, delete_files, expire_file, get_file_by_id, get_files, get_files_to_expire,
    update_file,
};
pub use integration::{Integration, get_integration_by_slug, get_integrations, upsert_integration};

pub use member::create_organisation_member;
#[cfg(feature = "web")]
pub use member::{get_organisation_member, update_organisation_member};

pub use organisation::{Organisation, get_default_organisation, get_organisation_for_user};

#[cfg(feature = "web")]
pub use onboarding::{get_onboarding_status, record_onboarding_completion};

pub use settings::get_organisation_settings;
#[cfg(feature = "web")]
pub use settings::update_organisation_settings;

pub use upload::{finish_upload, get_upload_by_file_id, preview_upload, start_upload};

pub use user::{User, create_user, get_user_by_email, get_user_by_id, get_user_password};
#[cfg(feature = "web")]
pub use user::{get_users, update_user_name};
