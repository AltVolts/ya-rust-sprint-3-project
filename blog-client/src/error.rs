use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogClientError {
    #[error("Http error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Grpc error: {0}")]
    GrpcError(tonic::Status),

    #[error("Transport error: {0}")]
    TransportError(tonic::transport::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid response from server: {0}")]
    InvalidResponse(String),
}

impl From<tonic::Status> for BlogClientError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::Unauthenticated => BlogClientError::Unauthorized,
            tonic::Code::NotFound => BlogClientError::NotFound,
            tonic::Code::InvalidArgument => {
                BlogClientError::InvalidRequest(status.message().to_string())
            }
            _ => BlogClientError::GrpcError(status),
        }
    }
}
