#[derive(Debug)]
pub enum Error {
    File(std::io::Error),
    Serde(serde_json::Error),
    ParseInt(std::num::ParseIntError)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::File(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}