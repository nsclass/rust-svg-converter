mod utils;
mod domain;
mod operation;

use image::*;
use domain::*;
use utils::*;
use operation::*;

struct ProgressListener { }

impl OperationProgressListener for ProgressListener {
    fn on_progress(&self, desc: &str, cur: usize, total: usize) {

    }
}

fn create_image_data(ctx: SvgConversionCtx) -> Result<(ImageData, ImageConvertOptions), String> {
    match ctx {
        SvgConversionCtx::Base64Image((base64, options)) => {
            let image_raw_data = image_base64::from_base64(base64);

            let res = image::load_from_memory(&image_raw_data);
            match res {
                Ok(dynamic_image) => {
                    let rgba_image = dynamic_image.to_rgba();
                    let image_data = ImageData::new(rgba_image.width() as usize, rgba_image.height() as usize, rgba_image.into_raw());
                    Result::Ok((image_data, options))
                },
                Err(image_error) => Result::Err(image_error.to_string()),
            }
        },
        _ => Result::Err(String::from("not valid base64 image string"))
    }
}

pub fn svg_converted_str_from_base64_image(base64: String) -> Result<String, String> {

    let listener = ProgressListener{};
    let mut operation_manager: OperationManager<SvgConversionCtx> = OperationManager::new(&listener);

    let options = ImageConvertOptions::default();
    let ctx = SvgConversionCtx::Base64Image((base64, options));

    // 1. loading an rgba image
    operation_manager.add_opeation("create an image data", |ctx| -> Result<SvgConversionCtx, String> {
        create_image_data(ctx).map(|(image_data, options)| SvgConversionCtx::ImageData((image_data, options)))
    });

    // 2. generate palette
    operation_manager.add_opeation("generate a palette", |ctx| -> Result<SvgConversionCtx, String> {
        generate_palette(ctx)
    });

    // 3. Color quantization
    operation_manager.add_opeation("color quantization", |ctx| -> Result<SvgConversionCtx, String> {
        color_quantization(ctx)
    });

    // execute all operations
    let res = operation_manager.execute(ctx);
    res.and_then(|ctx| ctx.into_svg_string().ok_or(String::from("Failed to convert an image")))
}
