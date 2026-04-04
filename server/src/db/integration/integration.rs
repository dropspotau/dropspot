use dropspot_core::integration::integration::Integration as ApiIntegration;
use serde::{Deserialize, Serialize};

use super::{get_gcs_integration, get_local_integration};

#[derive(Serialize, Deserialize)]
pub enum Integration {
    Local(LocalIntegration),
    Gcs(GcsIntegration),
}

pub async fn get_integrations(
    pool: &PgPool,
    organisation_id: &Uuid,
) -> Result<Option<LocalIntegration>, sqlx::Error> {
    let local_integration = get_local_integration(pool, organisation_id).await?;
    let gcs_integration = get_gcs_integration(pool, organisation_id).await?;

    Ok(vec![
        Integration::Local(local_integration),
        Integration::Gcs(gcs_integration),
    ])
}
