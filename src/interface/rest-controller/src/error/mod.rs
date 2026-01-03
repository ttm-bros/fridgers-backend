use fridgers_backend_use_case as use_case;

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
