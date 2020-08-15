// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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

    #[error("maximum supported number of quantization color is 256")]
    NotSupportedNumberOfColorForQuantization,

    #[error("failed to generate layers")]
    LayerGenerationFailure,

    #[error("failed to generate scan paths")]
    ScanPathGenerationFailure,

    #[error("failed to generate batch interpolation list")]
    BatchInterpolationGenerationFailure,

    #[error("failed image path tracing")]
    ImagePathTracingFailure,

    #[error("failed to generate svg string")]
    FailureGeneratingSvgString,

    #[error("unknown error")]
    Unknown,
}
