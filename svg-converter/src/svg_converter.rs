use crate::Error;

use crate::domain::ImageConvertOptions;
use crate::domain::ImageData;
use crate::domain::SvgConversionCtx;
use crate::operation::color_quantization;
use crate::operation::generate_palette;
use crate::utils::OperationManager;
use crate::utils::OperationProgressListener;

struct ProgressListener {}

impl OperationProgressListener for ProgressListener {
    fn on_progress(&self, desc: &str, cur: usize, total: usize) {
        println!("{}, {}, {}", desc, cur, total);
    }
}

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

pub fn svg_converted_str_from_base64_image(base64: String) -> Result<String, Error> {
    let listener = ProgressListener {};
    let mut operation_manager: OperationManager<SvgConversionCtx> =
        OperationManager::new(&listener);

    let options = ImageConvertOptions::default();
    let ctx = SvgConversionCtx::Base64Image((base64, options));

    // 1. loading an rgba image
    operation_manager.add_operation(
        "create an image data",
        |ctx| -> Result<SvgConversionCtx, Error> {
            create_image_data(ctx)
                .map(|(image_data, options)| SvgConversionCtx::ImageData((image_data, options)))
        },
    );

    // 2. generate palette
    operation_manager.add_operation(
        "generate a palette",
        |ctx| -> Result<SvgConversionCtx, Error> { generate_palette(ctx) },
    );

    // 3. Color quantization
    operation_manager.add_operation(
        "color quantization",
        |ctx| -> Result<SvgConversionCtx, Error> { color_quantization(ctx) },
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
