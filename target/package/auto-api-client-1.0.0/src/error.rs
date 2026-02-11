use std::fmt;

/// Error type for all client operations.
#[derive(Debug)]
pub enum Error {
    /// Authentication error (401/403).
    Auth {
        status_code: u16,
        message: String,
    },
    /// API error (other HTTP error codes).
    Api {
        status_code: u16,
        message: String,
        body: String,
    },
    /// Network/transport error (reqwest error).
    Network(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Auth {
                status_code,
                message,
            } => write!(f, "auth error {}: {}", status_code, message),
            Error::Api {
                status_code,
                message,
                ..
            } => write!(f, "API error {}: {}", status_code, message),
            Error::Network(e) => write!(f, "network error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Network(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e)
    }
}
