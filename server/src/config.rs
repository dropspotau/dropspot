// Server configuration, unused currently
#[derive(Clone)]
pub struct ServerConfiguration {
    pub local_upload_path: String,

    // Should the web portal show a disclaimer (don't bother with compilation feature flag for "web"
    // yet)
    pub should_show_disclaimer: bool,
}

pub fn get_server_config() -> ServerConfiguration {
    let local_upload_path = std::env::var("DROPSPOT_LOCAL_UPLOAD_PATH")
        .ok()
        .unwrap_or("files".to_owned());

    let should_show_disclaimer = std::env::var("DROPSPOT_WEB_SHOW_DISCLAIMER")
        .unwrap_or("false".to_owned())
        .parse::<bool>();

    if should_show_disclaimer.is_err() {
        tracing::error!(
            "Could not parse DROPSPOT_WEB_SHOW_DISCLAIMER as a boolean. Defaulting to false."
        );
    };

    let should_show_disclaimer = should_show_disclaimer.unwrap_or(false);

    ServerConfiguration {
        local_upload_path,
        should_show_disclaimer,
    }
}
