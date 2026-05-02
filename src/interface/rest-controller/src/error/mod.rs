use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use fridgers_backend_use_case as use_case;
use serde_json::json;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum Error {
    // 400
    InvalidArgument(String),

    // 401
    Unauthorized(String),

    // 403
    Forbidden(String),

    // 404
    NotFound(String),

    // 409
    AlreadyExist(String),

    // 412
    PreconditionFailed(String),

    // 500
    ExternalServer(String),
}

impl Error {
    /// エラーに対応するHTTPステータスコードを返す
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::Forbidden(_) => StatusCode::FORBIDDEN,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::AlreadyExist(_) => StatusCode::CONFLICT,
            Error::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            Error::ExternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<use_case::Error> for Error {
    fn from(err: use_case::Error) -> Self {
        match err {
            use_case::Error::InvalidArgument(msg) => Error::InvalidArgument(msg),
            use_case::Error::Unauthorized(msg) => Error::Unauthorized(msg),
            use_case::Error::Forbidden(msg) => Error::Forbidden(msg),
            use_case::Error::NotFound(msg) => Error::NotFound(msg),
            use_case::Error::AlreadyExist(msg) => Error::AlreadyExist(msg),
            use_case::Error::PreconditionFailed(msg) => Error::PreconditionFailed(msg),
            use_case::Error::ExternalServer(msg) => Error::ExternalServer(msg),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "error": self.to_string()
        }))
    }
}
