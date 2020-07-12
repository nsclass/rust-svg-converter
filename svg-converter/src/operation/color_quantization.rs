use crate::domain::QuantizationImageArray;
use crate::domain::SvgConversionCtx;
use failure::Error;

pub fn color_quantization(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((image_data, palette_data, options)) = ctx.into_palette_data() {
        let min_color_ratio = options.min_color_ratio;
        let cycles = options.color_quant_cycles;

        let mut quantization_data =
            QuantizationImageArray::zeros((image_data.height + 2, image_data.width + 2));

        for row in 0..image_data.height + 2 {
            quantization_data[[row, 0]] = -1;
            quantization_data[[row, image_data.width + 1]] = -1;
        }

        for col in 0..image_data.width + 2 {
            quantization_data[[0, col]] = -1;
            quantization_data[[image_data.height + 1, col]] = -1;
        }

        Ok(SvgConversionCtx::Quantization((
            quantization_data,
            palette_data,
            options,
        )))
    } else {
        failure::bail!("failed to create a color quantization")
    }
}
