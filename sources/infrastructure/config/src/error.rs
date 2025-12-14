use fridgers_backend_use_case as use_case;

pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Load(String),
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Self {
        Error::Load(err.to_string())
    }
}

impl From<dotenvy::Error> for Error {
    fn from(err: dotenvy::Error) -> Self {
        Error::Load(err.to_string())
    }
}

impl From<Error> for use_case::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Load(msg) => use_case::Error::ExternalServer(msg),
        }
    }
}
