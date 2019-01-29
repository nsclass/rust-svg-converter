
use super::domain::*;
use rand::Rng;

pub fn generate_palette(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, String> {
    ctx.into_image_data().map(|(image_data, options)| -> SvgConversionCtx {
        let palette = if options.color_sampling {
            generate_sample_palette(&image_data, options.number_of_colors)
        } else {
            generate_palette_data(options.number_of_colors)
        };
            
        SvgConversionCtx::Palette((image_data, palette, options))
    })
    .ok_or(String::from("failed to generate a palette"))
}

fn generate_sample_palette(image_data: &ImageData, number_of_colors: u32) -> ImageArray {
    let mut palette_data = ImageArray::zeros((number_of_colors as usize, 4));
    let mut rng = rand::thread_rng();

        for color_idx in 0 .. number_of_colors as usize {
            let idx = (rng.gen_range(0, image_data.len()) / 4) * 4;
            
            palette_data[[color_idx, 0]] = image_data.index_at(idx);
            palette_data[[color_idx, 1]] = image_data.index_at(idx + 1);
            palette_data[[color_idx, 2]] = image_data.index_at(idx + 2);
            palette_data[[color_idx, 3]] = image_data.index_at(idx + 3);
        }

    palette_data
}

fn generate_palette_data(number_of_colors: u32) -> ImageArray {

    let mut palette_data = ImageArray::zeros((number_of_colors as usize, 4));
    
    if number_of_colors < 8 {
        let gray_step = 255.0 / (number_of_colors - 1) as f32;
        for idx in 0..number_of_colors as usize {
            let value = (idx as f32 * gray_step).round() as u8;
            palette_data[[idx, 0]] = value;
            palette_data[[idx, 1]] = value;
            palette_data[[idx, 2]] = value;
            palette_data[[idx, 3]] = 255;
        }
    } else {
        let mut rng = rand::thread_rng();

        // RGB color cube
        let color_cube_count = f64::floor(f64::powf(number_of_colors as f64, 1.0 / 3.0)) as usize; // Number of points on each edge on the RGB color cube
        let color_step = f64::floor(255.0 / (color_cube_count - 1) as f64) as usize; // distance between points
        let mut color_count = 0;
        for red_idx in 0 .. color_cube_count {
            for green_idx in 0 .. color_cube_count {
                for blue_idx in 0 .. color_cube_count {
                    palette_data[[color_count, 0]] = (red_idx * color_step) as u8;
                    palette_data[[color_count, 1]] = (green_idx * color_step) as u8;
                    palette_data[[color_count, 2]] = (blue_idx * color_step) as u8;
                    palette_data[[color_count, 3]] = 255;
                    color_count += 1;
                }// End of blue loop
            }// End of green loop
        }// End of red loop

        // Rest is random
        for _ in 0 .. number_of_colors {
            palette_data[[color_count, 0]] = rng.gen_range(0, 255) as u8;
            palette_data[[color_count, 1]] = rng.gen_range(0, 255) as u8;
            palette_data[[color_count, 2]] = rng.gen_range(0, 255) as u8;
            palette_data[[color_count, 3]] = rng.gen_range(0, 255) as u8;
        }
    }

    palette_data
}
