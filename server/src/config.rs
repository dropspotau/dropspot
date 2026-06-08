// Server configuration, unused currently
#[derive(Clone)]
pub struct ServerConfiguration {
    pub local_upload_path: String,
}

pub fn get_server_config() -> ServerConfiguration {
    let local_upload_path = std::env::var("DROPSPOT_LOCAL_UPLOAD_PATH")
        .ok()
        .unwrap_or("files".to_owned());

    ServerConfiguration { local_upload_path }
}
