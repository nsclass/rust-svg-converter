use crate::domain::ImageConvertOptions;
use crate::domain::ImageData;
use crate::domain::{ImageArray, QuantizationImageArray};

pub enum SvgConversionCtx {
    Base64Image((String, ImageConvertOptions)),
    ImageData((ImageData, ImageConvertOptions)),
    Palette((ImageData, ImageArray, ImageConvertOptions)),
    Quantization((QuantizationImageArray, ImageArray, ImageConvertOptions)),
    SvgString(String),
}

impl SvgConversionCtx {
    pub fn into_base64_image(self) -> Option<(String, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::Base64Image((base64_image, options)) => Some((base64_image, options)),
            _ => None,
        }
    }

    pub fn into_image_data(self) -> Option<(ImageData, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::ImageData((image_data, options)) => Some((image_data, options)),
            _ => None,
        }
    }

    pub fn into_palette_data(self) -> Option<(ImageData, ImageArray, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::Palette((image_data, image_array, image_convert_options)) => {
                Some((image_data, image_array, image_convert_options))
            }
            _ => None,
        }
    }

    pub fn into_svg_string(self) -> Option<String> {
        match self {
            SvgConversionCtx::SvgString(svg_string) => Some(svg_string),
            _ => None,
        }
    }
}
