use std::io::{Read, Write};

use dropspot_core::user::refresh_tokens;

pub fn save_login(refresh_token: &str) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create("refresh_token")?;
    file.write(refresh_token.as_bytes()).map(|_size| ())
}

fn get_refresh_token() -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open("refresh_token")?;

    const TOKEN_LENGTH: usize = 275;
    let mut token = String::with_capacity(TOKEN_LENGTH);
    file.read_to_string(&mut token)?;

    Ok(token)
}

pub async fn get_access_token() -> Result<String, std::io::Error> {
    let refresh_token = get_refresh_token()?;
    let access_token = refresh_tokens(refresh_token)
        .await
        .expect("Could not refresh access token")
        .tokens
        .access_token;

    Ok(access_token)
}
