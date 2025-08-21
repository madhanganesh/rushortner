use axum::Json;
use axum::extract::{OriginalUri, Path, State};
use axum::http::StatusCode;
use axum::response::Redirect;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use rushortner::{base62_decode, base62_encode};

pub fn get_base_url(original_uri: &OriginalUri) -> String {
    let host = original_uri.0.host().unwrap_or("localhost");
    let scheme = original_uri.0.scheme_str().unwrap_or("http");
    let port_str = if let Some(port) = original_uri.0.port_u16() {
        format!(":{port}")
    } else {
        ":8081".to_string()
    };
    format!("{scheme}://{host}{port_str}")
}

pub async fn home() -> &'static str {
    "hello"
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub short_url: String,
}

pub async fn shorten_url(
    State(db): State<Pool<Postgres>>,
    original_uri: OriginalUri,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<UrlResponse>, (StatusCode, Json<Value>)> {
    if payload.url.is_empty() {
        let error_response = serde_json::json!({
            "error": "URL cannot be empty"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let id = sqlx::query_scalar!(
        r#"INSERT INTO urls (long_url) VALUES ($1) RETURNING id"#,
        payload.url
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let short_code = base62_encode(id.try_into().unwrap());
    let base_url = get_base_url(&original_uri);

    Ok(Json(UrlResponse {
        short_url: format!("{base_url}/{short_code}"),
    }))
}

pub async fn redirect_to_full_url(
    Path(code): Path<String>,
    State(db): State<Pool<Postgres>>,
) -> Result<Redirect, StatusCode> {
    let id_u64 = match base62_decode(&code) {
        Some(val) => val,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let id = id_u64 as i64;

    let long_url: Option<String> =
        sqlx::query_scalar!(r#"SELECT long_url FROM urls WHERE id = $1"#, id as i64)
            .fetch_optional(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(url) = long_url {
        Ok(Redirect::to(url.as_str()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
