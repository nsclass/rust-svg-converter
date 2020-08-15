// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::Error;

use crate::domain::ImageConvertOptions;
use crate::domain::ImageData;
use crate::domain::SvgConversionCtx;
use crate::utils::OperationManager;
use crate::{
    generate_batch_interpolation_list, generate_layer_edge_detection,
    generate_palette_quantization, generate_scan_paths, generate_svg_string, image_path_tracing,
    utils::OperationProgressListener,
};

fn create_image_data(ctx: SvgConversionCtx) -> Result<(ImageData, ImageConvertOptions), Error> {
    match ctx {
        SvgConversionCtx::Base64Image((base64, options)) => {
            let image_raw_data = image_base64::from_base64(base64);

            let res = image::load_from_memory(&image_raw_data);
            match res {
                Ok(dynamic_image) => {
                    let rgba_image = dynamic_image.to_rgba();
                    let image_data = ImageData::new(
                        rgba_image.height() as usize,
                        rgba_image.width() as usize,
                        rgba_image.into_raw(),
                    );
                    Result::Ok((image_data, options))
                }
                Err(image_error) => Err(Error::ImageError(image_error)),
            }
        }
        _ => Err(Error::NotValidBase64),
    }
}

pub fn svg_converted_str_from_base64_image(
    base64: String,
    progress_listener: &dyn OperationProgressListener,
) -> Result<String, Error> {
    let mut operation_manager: OperationManager<SvgConversionCtx> =
        OperationManager::new(progress_listener);

    let options = ImageConvertOptions::default();
    let ctx = SvgConversionCtx::Base64Image((base64, options));

    // 1. loading an rgba image
    operation_manager.add_operation(
        "create an image data",
        |ctx| -> Result<SvgConversionCtx, Error> {
            create_image_data(ctx).map(|(image_data, options)| {
                SvgConversionCtx::ImageData((image_data.into_color(), options))
            })
        },
    );

    // 2. generate palette and color quantization
    operation_manager.add_operation(
        "generate palette and color quantization",
        |ctx| -> Result<SvgConversionCtx, Error> { generate_palette_quantization(ctx) },
    );

    // 3. generate layers and edge detection
    operation_manager.add_operation(
        "generate layers and edge detection",
        |ctx| -> Result<SvgConversionCtx, Error> { generate_layer_edge_detection(ctx) },
    );

    // 4. scan paths
    operation_manager.add_operation("scan paths", |ctx| -> Result<SvgConversionCtx, Error> {
        generate_scan_paths(ctx)
    });

    // 5. batch interpolation
    operation_manager.add_operation(
        "batch interpolation",
        |ctx| -> Result<SvgConversionCtx, Error> { generate_batch_interpolation_list(ctx) },
    );

    // 6. image path tracing
    operation_manager.add_operation(
        "image path tracing",
        |ctx| -> Result<SvgConversionCtx, Error> { image_path_tracing(ctx) },
    );

    // 7. generate svg string
    operation_manager.add_operation(
        "generate svg string",
        |ctx| -> Result<SvgConversionCtx, Error> { generate_svg_string(ctx) },
    );

    // execute all operations
    let res = operation_manager.execute(ctx);
    res.and_then(|ctx| {
        if let Some(svg_str) = ctx.into_svg_string() {
            Ok(svg_str)
        } else {
            Err(Error::ImageConvertFailure)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE_BASE64: &'static str =
        include_str!("../../svg-web-service/tests/bdd/features/samples/image_sample_base64.txt");

    #[test]
    fn svg_conversion_success() {
        struct ProgressListener {}

        impl OperationProgressListener for ProgressListener {
            fn on_progress(&self, desc: &str, cur: usize, total: usize) {
                println!("progress: {}, {}/{}", desc, cur + 1, total);
            }
        }
        // let data = std::fs::read_to_string("/etc/hosts").expect("Unable to read file");
        let res =
            svg_converted_str_from_base64_image(SAMPLE_BASE64.to_string(), &ProgressListener {});
        match res {
            Ok(svg_string) => {
                assert!(svg_string.len() > 0);
                // std::fs::write("/Users/nsclass/tmp/output.svg", svg_string)
                //     .expect("Unable to write file");
            }
            Err(err) => assert!(false, format!("failed to generate a svg image: {}", err)),
        }
    }
}
