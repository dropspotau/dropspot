use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Authentication {
    pub token: String,
}

pub fn get_auth_headers(auth: Option<&Authentication>) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if let Some(ref auth) = auth {
        headers.insert(
            "Authorization",
            format!("Token {}", auth.token).parse().unwrap(),
        );
    }

    headers
}
