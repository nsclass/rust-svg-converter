use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ImageError(image::error::ImageError),

    #[error("not valid base64 image string")]
    NotValidBase64,
    #[error("failed to convert an image")]
    ImageConvertFailure,
    #[error("out of index on finding gaussian")]
    GaussianIndexError,

    #[error("failed to create a color quantization")]
    FailureColorQuantization,

    #[error("failed to generate a palette")]
    FailureGeneratePallette,

    #[error("unknown error")]
    Unknown,
}
