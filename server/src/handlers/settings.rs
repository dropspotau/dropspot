use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use dropspot_core::settings::{Settings as ApiSettings, UpdateSettingsPayload};
use reqwest::StatusCode;

use crate::{
    db::{
        Settings, User, get_organisation_for_user, get_organisation_member,
        update_organisation_settings,
    },
    state::AppState,
    types::ApiError,
};

impl From<Settings> for ApiSettings {
    fn from(settings: Settings) -> Self {
        Self {
            default_file_expiry_minutes: settings.default_file_expiry_minutes,
            default_download_limit: settings.default_download_limit,
            allow_external_uploads: settings.allow_external_uploads,
            allow_external_downloads: settings.allow_external_downloads,
        }
    }
}

pub async fn handle_update_settings(
    State(state): State<AppState>,
    user: User,
    Json(payload): Json<UpdateSettingsPayload>,
) -> Response {
    let pool = state.get_pool();
    let Ok(organisation) = get_organisation_for_user(pool, &user.id).await else {
        return ApiError::new(
            "Organisation not found".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    };

    let Ok(member) = get_organisation_member(pool, &organisation.id, &user.id).await else {
        return ApiError::new(
            "Organisation membership not found".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    };

    let can_edit = member.is_admin;
    if !can_edit || !payload.validate() {
        return ApiError::new(
            "Invalid settings update payload".to_owned(),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    let Ok(settings) = update_organisation_settings(
        pool,
        &organisation.id,
        payload.file_expiry_minutes,
        payload.download_limit,
        payload.allow_external_uploads,
        payload.allow_external_downloads,
    )
    .await
    else {
        return ApiError::new(
            "Sorry, there was an error updating organisation settings. Please try again."
                .to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

    Json(settings).into_response()
}
