// Server configuration, unused currently
#[derive(Clone)]
pub struct ServerConfiguration {
    pub local_upload_path: String,

    // Should the web portal show a disclaimer and contact (don't bother with compilation feature flag for "web"
    // yet)
    pub should_show_contact: bool,
}

pub fn get_server_config() -> ServerConfiguration {
    let local_upload_path = match std::env::var("DROPSPOT_LOCAL_UPLOAD_PATH").ok() {
        Some(path) => path,
        None => {
            tracing::warn!("DROPSPOT_LOCAL_UPLOAD_PATH variable missing. Defaulting to \"files\".");
            "files".to_owned()
        }
    };

    let should_show_contact =
        std::env::var("DROPSPOT_WEB_SHOW_CONTACT").map(|v| v.parse::<bool>().unwrap_or(false));

    if should_show_contact.is_err() {
        tracing::warn!(
            "Could not parse DROPSPOT_WEB_SHOW_CONTACT as a boolean. Defaulting to false."
        );
    };

    let should_show_contact = should_show_contact.unwrap_or(false);

    ServerConfiguration {
        local_upload_path,
        should_show_contact,
    }
}
