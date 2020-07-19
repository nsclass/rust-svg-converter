use crate::domain::{ImageArray, ImageData, QuantizationImageArray, SvgConversionCtx};
use crate::Error;

// double[][] GAUSSIAN_KERNEL_FOR_BLUR = {
//     {0.27901,  0.44198,  0.27901 },
//     {0.135336, 0.228569, 0.272192, 0.228569, 0.135336},
//     {0.086776, 0.136394, 0.178908, 0.195843, 0.178908, 0.136394, 0.086776},
//     {0.063327, 0.093095, 0.122589, 0.144599, 0.152781, 0.144599, 0.122589, 0.093095, 0.063327},
//     {0.049692, 0.069304, 0.089767, 0.107988, 0.120651, 0.125194, 0.120651, 0.107988, 0.089767, 0.069304, 0.049692}
// };

fn get_gaussian_kernel_for_blur(idx: usize) -> Result<&'static [f32], Error> {
    static VALUE_1: &'static [f32] = &[0.27901, 0.44198, 0.27901];
    static VALUE_2: &'static [f32] = &[0.135336, 0.228569, 0.272192, 0.228569, 0.135336];
    static VALUE_3: &'static [f32] = &[
        0.086776, 0.136394, 0.178908, 0.195843, 0.178908, 0.136394, 0.086776,
    ];
    static VALUE_4: &'static [f32] = &[
        0.063327, 0.093095, 0.122589, 0.144599, 0.152781, 0.144599, 0.122589, 0.093095, 0.063327,
    ];
    static VALUE_5: &'static [f32] = &[
        0.049692, 0.069304, 0.089767, 0.107988, 0.120651, 0.125194, 0.120651, 0.107988, 0.089767,
        0.069304, 0.049692,
    ];

    static VALUE_HOLDERS: &'static [&'static [f32]] =
        &[VALUE_1, VALUE_2, VALUE_3, VALUE_4, VALUE_5];

    if idx < VALUE_HOLDERS.len() {
        Ok(VALUE_HOLDERS[idx])
    } else {
        Err(Error::GaussianIndexError)
    }
}

pub fn create_blur_image(
    image_data: ImageData,
    blur_radius: f32,
    delta: f32,
) -> Result<ImageData, Error> {
    let mut image_raw_data = vec![0 as u8; image_data.width * image_data.height * 4];
    let mut usize_radius = blur_radius.floor() as i32;
    if usize_radius < 1 {
        return Ok(image_data);
    }

    if usize_radius > 5 {
        usize_radius = 5;
    }

    let mut delta_usize = delta.abs() as i32;
    if delta_usize > 1024 {
        delta_usize = 1024;
    }

    let found_gk = get_gaussian_kernel_for_blur((usize_radius - 1) as usize)?;

    for row in 0..image_data.height as i32 {
        for col in 0..image_data.width as i32 {
            let mut r_acc = 0. as f32;
            let mut g_acc = 0. as f32;
            let mut b_acc = 0. as f32;
            let mut a_acc = 0. as f32;
            let mut w_acc = 0. as f32;

            // gaussian kernel loop
            for k in -1 as i32..(usize_radius + 1) as i32 {
                if (col + k) > 0 && (col + k) < image_data.width as i32 {
                    let idx = (((row * image_data.width as i32) + col + k) * 4) as usize;
                    r_acc +=
                        image_data.index_at(idx) as f32 * found_gk[(k + usize_radius) as usize];
                    g_acc +=
                        image_data.index_at(idx + 1) as f32 * found_gk[(k + usize_radius) as usize];
                    b_acc +=
                        image_data.index_at(idx + 2) as f32 * found_gk[(k + usize_radius) as usize];
                    a_acc +=
                        image_data.index_at(idx + 3) as f32 * found_gk[(k + usize_radius) as usize];
                    w_acc += found_gk[(k + usize_radius) as usize];
                }
            }

            // the new pixel
            let idx = (((row * image_data.width as i32) + col) * 4) as usize;
            image_raw_data[idx] = (r_acc / w_acc).floor() as u8;
            image_raw_data[idx + 1] = (g_acc / w_acc).floor() as u8;
            image_raw_data[idx + 2] = (b_acc / w_acc).floor() as u8;
            image_raw_data[idx + 3] = (a_acc / w_acc).floor() as u8;
        }
    }

    // copying the half blurred image data
    let half_image_data = image_raw_data.clone();

    // loop through all pixels for vertical blur
    for row in 0..image_data.height as i32 {
        for col in 0..image_data.width as i32 {
            let mut r_acc = 0. as f32;
            let mut g_acc = 0. as f32;
            let mut b_acc = 0. as f32;
            let mut a_acc = 0. as f32;
            let mut w_acc = 0. as f32;

            // gaussian kernel loop
            for k in -(usize_radius as i32)..(usize_radius + 1) as i32 {
                if (row + k) > 0 && (row + k) < image_data.height as i32 {
                    let idx = ((((row + k) * image_data.width as i32) + col) * 4) as usize;
                    r_acc += half_image_data[idx] as f32 * found_gk[(k + usize_radius) as usize];
                    g_acc +=
                        half_image_data[idx + 1] as f32 * found_gk[(k + usize_radius) as usize];
                    b_acc +=
                        half_image_data[idx + 2] as f32 * found_gk[(k + usize_radius) as usize];
                    a_acc +=
                        half_image_data[idx + 3] as f32 * found_gk[(k + usize_radius) as usize];
                    w_acc += found_gk[(k + usize_radius) as usize];
                }
            }

            // the new pixel
            let idx = (((row * image_data.width as i32) + col) * 4) as usize;
            image_raw_data[idx] = (r_acc / w_acc).floor() as u8;
            image_raw_data[idx + 1] = (g_acc / w_acc).floor() as u8;
            image_raw_data[idx + 2] = (b_acc / w_acc).floor() as u8;
            image_raw_data[idx + 3] = (a_acc / w_acc).floor() as u8;
        }
    }
    // selective blue: loop through all pixels

    for row in 0..image_data.height as i32 {
        for col in 0..image_data.width as i32 {
            let idx = (((row * image_data.width as i32) + col) * 4) as usize;
            let d = (image_raw_data[idx] as i32 - image_data.index_at(idx) as i32).abs()
                + (image_raw_data[idx + 1] as i32 - image_data.index_at(idx + 1) as i32).abs()
                + (image_raw_data[idx + 2] as i32 - image_data.index_at(idx + 2) as i32).abs()
                + (image_raw_data[idx + 3] as i32 - image_data.index_at(idx + 3) as i32).abs();

            if d > delta as i32 {
                image_raw_data[idx] = image_data.index_at(idx);
                image_raw_data[idx + 1] = image_data.index_at(idx + 1);
                image_raw_data[idx + 2] = image_data.index_at(idx + 2);
                image_raw_data[idx + 3] = image_data.index_at(idx + 3);
            }
        }
    }

    Ok(ImageData::new(
        image_data.width,
        image_data.height,
        image_raw_data,
    ))
}

pub fn color_quantization(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((mut image_data, mut palette_data, options)) = ctx.into_palette_data() {
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

        // selective Gaussian blur preprocessing
        if options.blur_radius > 0. {
            image_data = create_blur_image(image_data, options.blur_radius, options.blur_delta)?;
        }

        let mut palette_acc = QuantizationImageArray::zeros((palette_data.shape()[0], 5));
        // repeat clustering step cycles times

        for loop_idx in 0..cycles {
            color_quantization_iterate(
                loop_idx,
                min_color_ratio,
                cycles,
                &mut quantization_data,
                &mut palette_acc,
                &image_data,
                &mut palette_data,
            );
        }
        Ok(SvgConversionCtx::Quantization((
            quantization_data,
            palette_data,
            options,
        )))
    } else {
        Err(Error::FailureColorQuantization)
    }
}

fn color_quantization_iterate(
    loop_idx: u32,
    min_color_ratio: f32,
    cycles: u32,
    quantization_data: &mut QuantizationImageArray,
    palette_acc: &mut QuantizationImageArray,
    image_data: &ImageData,
    palette_data: &mut ImageArray,
) {
}
