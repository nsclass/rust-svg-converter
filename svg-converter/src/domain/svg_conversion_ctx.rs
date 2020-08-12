use crate::domain::ImageConvertOptions;
use crate::domain::ImageData;
use crate::{ImageColorData, ScanPathList};
use exoquant::Color;

pub enum SvgConversionCtx {
    Base64Image((String, ImageConvertOptions)),
    ImageData((ImageColorData, ImageConvertOptions)),
    ColorQuantization((Vec<Color>, ImageData, ImageConvertOptions)),
    Layers((Vec<Vec<Vec<i32>>>, ImageConvertOptions)),
    ScanPaths((ScanPathList, ImageConvertOptions)),
    SvgString(String),
}

impl SvgConversionCtx {
    pub fn into_base64_image(self) -> Option<(String, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::Base64Image((base64_image, options)) => Some((base64_image, options)),
            _ => None,
        }
    }

    pub fn into_image_data(self) -> Option<(ImageColorData, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::ImageData((image_data, options)) => Some((image_data, options)),
            _ => None,
        }
    }

    pub fn into_quantization_data(self) -> Option<(Vec<Color>, ImageData, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::ColorQuantization((palette, indexed_image, options)) => {
                Some((palette, indexed_image, options))
            }
            _ => None,
        }
    }

    pub fn into_layers(self) -> Option<(Vec<Vec<Vec<i32>>>, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::Layers((layers, options)) => Some((layers, options)),
            _ => None,
        }
    }

    pub fn into_scan_paths(self) -> Option<(ScanPathList, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::ScanPaths((scan_paths, options)) => Some((scan_paths, options)),
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
