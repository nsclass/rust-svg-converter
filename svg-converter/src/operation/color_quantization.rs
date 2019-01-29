
use super::domain::*;

pub fn color_quantization(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, String> {
    ctx.into_palette_data().map(|(image_data, palette_data, options)| -> SvgConversionCtx {

        let min_color_ratio = options.min_color_ratio;
        let cycles = options.color_quant_cycles;

        let mut quantization_data = QuantizationImageArray::zeros((image_data.height + 2, image_data.width + 2));

        for row in 0 .. image_data.height + 2 {
            quantization_data[[row, 0]] = -1;
            quantization_data[[row, image_data.width + 1]] = -1;
        }

        for col in 0 .. image_data.width + 2 {
            quantization_data[[0, col]] = -1;
            quantization_data[[image_data.height + 1, col]] = -1;
        }

        SvgConversionCtx::Quantization((quantization_data, palette_data, options))
    })
    .ok_or(String::from("failed to create a color quantization"))
}