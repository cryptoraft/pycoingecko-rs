#[derive(Debug)]
pub enum ApiError {
    ClientInitializationError(String),
    RequestError(String),
    ParsingError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::ClientInitializationError(msg) => write!(f, "Client Initialization Error: {}", msg),
            ApiError::RequestError(msg) => write!(f, "Request Error: {}", msg),
            ApiError::ParsingError(msg) => write!(f, "Parsing Error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}
