use std::io::Write;

pub fn save_login(refresh_token: &str) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create("refresh_token")?;
    file.write(refresh_token.as_bytes()).map(|_size| ())
}
