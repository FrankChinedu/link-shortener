use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use metrics::counter;
use sha3::{Digest, Sha3_256};
use sqlx::PgPool;

use crate::utils::internal_error;

struct Setting {
    #[allow(dead_code)]
    id: String,
    encrypted_global_api_key: String,
}

pub async fn auth(
    State(pool): State<PgPool>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let labels = [("uri", format!("{}!", req.uri()))];

    let api_key = req
        .headers()
        .get("x-api-key")
        .map(|value| value.to_str().unwrap_or_default())
        .ok_or_else(|| {
            tracing::error!("Unauthorized call to API: No key header received");
            let counter = counter!("unauthenticated_calls_count", &labels);
            counter.increment(1);

            (StatusCode::UNAUTHORIZED, "Unathorized".into())
        })?;

    let fetch_setting_timeout = tokio::time::Duration::from_millis(300);

    let setting = tokio::time::timeout(
        fetch_setting_timeout,
        sqlx::query_as!(
            Setting,
            "select id, encrypted_global_api_key from settings where id = $1",
            "DEFAULT_SETTINGS"
        )
        .fetch_one(&pool),
    )
    .await
    .map_err(internal_error)?
    .map_err(internal_error)?;

    let mut hasher = Sha3_256::new();
    hasher.update(api_key.as_bytes());
    let provided_api_key = hasher.finalize();

    if setting.encrypted_global_api_key != format!("{provided_api_key:x}") {
        tracing::error!("Unauthenticated call to API: Incorrect key supplied");

        let counter = counter!("unauthenticated_calls_count", &labels);
        counter.increment(1);

        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }

    Ok(next.run(req).await)
}
