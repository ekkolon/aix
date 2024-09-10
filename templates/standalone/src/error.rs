use actix_web::{http::StatusCode, HttpResponse, ResponseError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // --- Externals
    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),

    #[error(transparent)]
    DotEnv(#[from] dotenvy::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::ActixWeb(err) => err.as_response_error().status_code(),
            Error::DotEnv(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
