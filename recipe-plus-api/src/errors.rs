use axum::http::StatusCode;

#[cfg(debug_assertions)]
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

#[cfg(not(debug_assertions))]
pub fn internal_error<E>(_: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error".to_string(),
    )
}
