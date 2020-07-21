//! Crate error type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("stellar base error")]
    StellarBaseError(#[from] stellar_base::error::Error),
    #[error("sse decoder error")]
    SSEDecoderError,
    #[error("horizon request error")]
    HorizonRequestError(crate::horizon_error::HorizonError),
    #[error("horizon server error")]
    HorizonServerError,
    #[error("http error")]
    HttpError(#[from] http::Error),
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    #[error("json error")]
    JsonError(#[from] serde_json::error::Error),
    #[error("invalid uri")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("invalid url")]
    InvalidUrl(#[from] url::ParseError),
    #[error("invalid host")]
    InvalidHost,
}
