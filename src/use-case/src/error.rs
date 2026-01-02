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
