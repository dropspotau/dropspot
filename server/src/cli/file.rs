use std::{
    fs::File,
    io::{BufWriter, Read},
};

use base64::{
    Engine,
    alphabet::URL_SAFE,
    engine::{GeneralPurpose, general_purpose::NO_PAD},
};
use dropspot_core::{
    auth::Authentication,
    download::download,
    encryption::Encryption,
    file::{get_file, list_files},
    storage::StorageType,
    upload::upload,
    validation::validate_file,
};
use uuid::Uuid;

use crate::auth::storage::get_access_token;

pub async fn handle_upload(file_name: &str) -> Result<(), ()> {
    // Simulate generating an upload URL
    let validation = validate_file(file_name);

    if let Err(e) = validation {
        eprintln!("Failed to validate file: {e:?}");
        return Err(());
    }

    let mut file = validation.unwrap();
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        eprintln!("Failed to upload file: {e:?}");
        return Err(());
    }

    let authentication = match get_access_token().await {
        Ok(access_token) => Some(Authentication {
            token: access_token,
        }),
        Err(_e) => None,
    };

    let upload = upload(
        file_name.to_owned(),
        buffer,
        authentication,
        StorageType::Local,
    )
    .await;

    if let Err(e) = upload {
        eprintln!("Failed to upload file: {e:?}");
        return Err(());
    }

    let upload = upload.unwrap();

    let engine = GeneralPurpose::new(&URL_SAFE, NO_PAD);
    let key_base64 = engine.encode(&upload.encryption.key);
    let nonce_base64 = engine.encode(&upload.encryption.nonce);

    tracing::info!("Uploaded file {}", &upload.file.id);
    tracing::info!("Key: {key_base64}");
    tracing::info!("Key: {:?}", upload.encryption.key);
    tracing::info!("Nonce: {nonce_base64}");
    tracing::info!("Nonce: {:?}", upload.encryption.nonce);

    tracing::info!(
        "cargo run file download {} {key_base64} {nonce_base64}",
        upload.file.id
    );

    Ok(())
}

pub async fn handle_download(file_id: &Uuid, key: &str, nonce: &str) -> Result<(), ()> {
    let authentication = match get_access_token().await {
        Ok(access_token) => Some(Authentication {
            token: access_token,
        }),
        Err(_e) => None,
    };

    let engine = GeneralPurpose::new(&URL_SAFE, NO_PAD);
    let key = engine.decode(key).unwrap();
    let nonce = engine.decode(nonce).unwrap();

    let encryption = Encryption { key, nonce };

    let Ok(file) = get_file(file_id, authentication.as_ref()).await else {
        eprintln!("Failed to retrieve file details");
        return Err(());
    };

    // Decrypt the file
    let local_file_name = format!("download_{}", &file.name);
    let Ok(local_file) = File::create(&local_file_name) else {
        eprintln!("Failed to open local file to save");
        return Err(());
    };

    let stream_writer = BufWriter::new(local_file);
    if let Err(e) = download(file_id, &encryption, stream_writer, authentication.as_ref()).await {
        eprintln!("Failed to download file: {e}");
        return Err(());
    }

    println!("Download complete");
    Ok(())
}

pub async fn handle_list_files() -> Result<(), ()> {
    let authentication = match get_access_token().await {
        Ok(access_token) => Some(Authentication {
            token: access_token,
        }),
        Err(_e) => None,
    };

    let files = list_files(authentication.as_ref()).await;

    if let Err(e) = files {
        eprintln!("Failed to list files: {e}");
        return Err(());
    }

    let files = files.unwrap();
    println!("{files:?}");

    Ok(())
}

pub async fn handle_get_file(file_id: &Uuid) -> Result<(), ()> {
    let authentication = match get_access_token().await {
        Ok(access_token) => Some(Authentication {
            token: access_token,
        }),
        Err(_e) => None,
    };

    let file = get_file(file_id, authentication.as_ref()).await;

    if let Err(e) = file {
        eprintln!("Failed to get file: {e}");
        return Err(());
    }

    let file = file.unwrap();
    println!("{file:?}");

    Ok(())
}
