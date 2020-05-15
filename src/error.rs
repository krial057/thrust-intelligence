use serde::export::Formatter;

pub type MispResult<T> = std::result::Result<T, MispError>;

#[derive(Debug)]
pub enum MispError {
    UrlParseError(url::ParseError),
    HttpError(surf::Error),
    JsonError(serde_json::error::Error),
}

impl std::fmt::Display for MispError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for MispError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use MispError::*;
        match self {
            JsonError(e) => Some(e),
            UrlParseError(e) => Some(e),
            //HttpError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<url::ParseError> for MispError {
    fn from(value: url::ParseError) -> Self {
        MispError::UrlParseError(value)
    }
}

impl From<surf::Error> for MispError {
    fn from(value: surf::Error) -> Self {
        MispError::HttpError(value)
    }
}

impl From<serde_json::error::Error> for MispError {
    fn from(value: serde_json::error::Error) -> Self {
        MispError::JsonError(value)
    }
}
