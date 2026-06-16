use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::http::HeaderMap;
use reqwest::header::FORWARDED;
use sqlx::PgPool;

use crate::db::{Organisation, User, get_default_organisation, get_organisation_for_user};

pub async fn get_organisation_from_request_user(
    pool: &PgPool,
    user: Option<&User>,
) -> Result<Organisation, sqlx::Error> {
    match user {
        Some(u) => get_organisation_for_user(pool, &u.id).await,
        None => get_default_organisation(pool).await,
    }
}

// Retrieves the proxy-forwarded IP address, or the direct connection address if the header is
// missing
pub fn extract_client_ip(address: SocketAddr, headers: HeaderMap) -> IpAddr {
    if let Some(forwarded_for_ip) = headers.get(FORWARDED)
        && let Ok(forwarded_for_ip) = forwarded_for_ip.to_str()
        && let Ok(upload_ip) = IpAddr::from_str(forwarded_for_ip)
    {
        // NOTE(alec): This kind of sucks. It doesn't validate anything and just stores an IP which
        // could be spoofed
        return upload_ip;
    }

    // If the server isn't behind a reverse proxy, this will be the request's IP address
    address.ip()
}
