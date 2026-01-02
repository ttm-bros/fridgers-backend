pub type Result<A> = std::result::Result<A, Error>;

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum Error {
    InvalidFormat(String),
    InvalidLengthRange(String),
}
