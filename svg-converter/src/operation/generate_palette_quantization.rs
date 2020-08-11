use crate::{Error, ImageData, SvgConversionCtx};
use exoquant::{convert_to_indexed, ditherer, optimizer};

pub fn generate_palette_quantization(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((image_data, options)) = ctx.into_image_data() {
        if options.number_of_colors <= 256 {
            let (palette, indexed_data) = convert_to_indexed(
                &image_data.pixels,
                image_data.width,
                options.number_of_colors as usize,
                &optimizer::KMeans,
                &ditherer::FloydSteinberg::new(),
            );

            let indexed_image = ImageData::new(image_data.height, image_data.width, indexed_data);
            return Ok(SvgConversionCtx::ColorQuantization((
                palette,
                indexed_image,
                options,
            )));
        } else {
            return Err(Error::NotSupportedNumberOfColorForQuantization);
        }
    }
    Err(Error::FailureColorQuantization)
}

#[cfg(test)]
mod tests {
    use exoquant::{convert_to_indexed, ditherer, optimizer, testdata};

    #[test]
    fn image_quantization_creation() {
        // let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        // let image_data = ImageData::new(2, 4, data);
        // let palette = vec![Color::new(0, 0, 0, 0)];

        // let image_quantization = ImageQuantization {
        //     indexed_image: image_data,
        //     palette,
        // };
        let image = testdata::test_image();

        let (palette, indexed_data) = convert_to_indexed(
            &image.pixels,
            image.width,
            256,
            &optimizer::KMeans,
            &ditherer::FloydSteinberg::new(),
        );
        assert_eq!(image.pixels.len(), 65536);
        assert_eq!(palette.len(), 256);
        assert_eq!(indexed_data.len(), 65536);
    }
}
