// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::domain::ImageConvertOptions;
use crate::domain::ImageData;
use crate::{ImageColorData, ImagePathTraceLayers, InterpolationBatchList, ScanPathList};
use exoquant::Color;

pub enum SvgConversionCtx {
    Base64Image((String, ImageConvertOptions)),
    ImageData((ImageColorData, ImageConvertOptions)),
    ColorQuantization((Vec<Color>, ImageData, ImageConvertOptions)),
    Layers(
        (
            Vec<Color>,
            ImageData,
            Vec<Vec<Vec<i32>>>,
            ImageConvertOptions,
        ),
    ),
    ScanPaths((Vec<Color>, ImageData, ScanPathList, ImageConvertOptions)),
    BatchInterpolation(
        (
            Vec<Color>,
            ImageData,
            InterpolationBatchList,
            ImageConvertOptions,
        ),
    ),
    ImagePathTraceLayers(
        (
            Vec<Color>,
            ImageData,
            ImagePathTraceLayers,
            ImageConvertOptions,
        ),
    ),
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

    pub fn into_layers(
        self,
    ) -> Option<(
        Vec<Color>,
        ImageData,
        Vec<Vec<Vec<i32>>>,
        ImageConvertOptions,
    )> {
        match self {
            SvgConversionCtx::Layers((palette, indexed_image, layers, options)) => {
                Some((palette, indexed_image, layers, options))
            }
            _ => None,
        }
    }

    pub fn into_scan_paths(
        self,
    ) -> Option<(Vec<Color>, ImageData, ScanPathList, ImageConvertOptions)> {
        match self {
            SvgConversionCtx::ScanPaths((palette, indexed_image, scan_paths, options)) => {
                Some((palette, indexed_image, scan_paths, options))
            }
            _ => None,
        }
    }

    pub fn into_batch_interpolation(
        self,
    ) -> Option<(
        Vec<Color>,
        ImageData,
        InterpolationBatchList,
        ImageConvertOptions,
    )> {
        match self {
            SvgConversionCtx::BatchInterpolation((palette, indexed_image, batch_list, options)) => {
                Some((palette, indexed_image, batch_list, options))
            }
            _ => None,
        }
    }

    pub fn into_image_path_trace(
        self,
    ) -> Option<(
        Vec<Color>,
        ImageData,
        ImagePathTraceLayers,
        ImageConvertOptions,
    )> {
        match self {
            SvgConversionCtx::ImagePathTraceLayers((palette, indexed_image, layers, options)) => {
                Some((palette, indexed_image, layers, options))
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
