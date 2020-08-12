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
            // create an index image which has a boundary filled with 255
            // TODO: should find a better way instead of copying each cell.
            let height = image_data.height + 2;
            let width = image_data.width + 2;
            let mut indexed_image = ImageData::new(height, width, vec![0; height * width]);
            for row in 0..height {
                indexed_image[row][0] = 0xff;
                indexed_image[row][width - 1] = 0xff;
            }
            for col in 0..width {
                indexed_image[0][col] = 0xff;
                indexed_image[height - 1][col] = 0xff;
            }

            for row in 0..image_data.height {
                for col in 0..image_data.width {
                    let data = indexed_data[row * image_data.width + col];
                    indexed_image[row + 1][col + 1] = data;
                }
            }

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
    use crate::{
        generate_palette_quantization, ImageColorData, ImageConvertOptions, SvgConversionCtx,
    };
    use exoquant::Color;

    #[test]
    fn image_quantization_creation() {
        let image = ImageColorData {
            pixels: (0..100)
                .map(|i| {
                    let x = i & 255;
                    let y = i >> 8;
                    Color::new(x as u8, y as u8, 0, 255)
                })
                .collect(),
            height: 10,
            width: 10,
        };

        let ctx = SvgConversionCtx::ImageData((image, ImageConvertOptions::default()));
        let res = generate_palette_quantization(ctx);
        match res {
            Ok(SvgConversionCtx::ColorQuantization((palette, image, options))) => {
                assert_eq!(palette.len(), 16);
                assert_eq!(image.len(), 144);
                for row in 0..image.height {
                    assert_eq!(image[row][0], 0xff);
                    assert_eq!(image[row][image.width - 1], 0xff);
                }
                for col in 0..image.width {
                    assert_eq!(image[0][col], 0xff);
                    assert_eq!(image[image.height - 1][col], 0xff);
                }
            }

            _ => todo!(),
        }
    }
}
