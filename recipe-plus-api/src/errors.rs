use axum::http::StatusCode;

#[cfg(debug_assertions)]
pub fn diesel_error(err: diesel::result::Error) -> (StatusCode, String)
{
    match err {
        diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, err.to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

#[cfg(not(debug_assertions))]
pub fn diesel_error(err: diesel::result::Error) -> (StatusCode, String)
{
    match err {
        diesel::result::Error::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
    }
}

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
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
}
