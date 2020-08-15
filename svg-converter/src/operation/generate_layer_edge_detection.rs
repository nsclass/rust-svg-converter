// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{Error, SvgConversionCtx};

pub fn generate_layer_edge_detection(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, options)) = ctx.into_quantization_data() {
        // let x = vec![vec![vec![0.0f64; P]; N]; M];
        // double[][][] x = new double[m][n][p];
        // int[][][] layers = new int[palette.length][height][width];

        let palette_len = palette.len();
        let height = indexed_image.height;
        let width = indexed_image.width;
        let mut layers = vec![vec![vec![0; width]; height]; palette_len];

        for row in 1..height - 1 {
            for col in 1..width - 1 {
                // This pixel's indexed color
                let val = indexed_image[row][col];

                // Are neighbor pixel colors the same?
                let n1 = if indexed_image[row - 1][col - 1] == val {
                    1
                } else {
                    0
                };
                let n2 = if indexed_image[row - 1][col] == val {
                    1
                } else {
                    0
                };
                let n3 = if indexed_image[row - 1][col + 1] == val {
                    1
                } else {
                    0
                };
                let n4 = if indexed_image[row][col - 1] == val {
                    1
                } else {
                    0
                };
                let n5 = if indexed_image[row][col + 1] == val {
                    1
                } else {
                    0
                };
                let n6 = if indexed_image[row + 1][col - 1] == val {
                    1
                } else {
                    0
                };
                let n7 = if indexed_image[row + 1][col] == val {
                    1
                } else {
                    0
                };
                let n8 = if indexed_image[row + 1][col + 1] == val {
                    1
                } else {
                    0
                };

                // this pixel"s type and looking back on previous pixels
                layers[val as usize][row + 1][col + 1] = 1 + (n5 * 2) + (n8 * 4) + (n7 * 8);
                if n4 == 0 {
                    layers[val as usize][row + 1][col] = 0 + 2 + (n7 * 4) + (n6 * 8);
                }
                if n2 == 0 {
                    layers[val as usize][row][col + 1] = 0 + (n3 * 2) + (n5 * 4) + 8;
                }
                if n1 == 0 {
                    layers[val as usize][row][col] = 0 + (n2 * 2) + 4 + (n4 * 8);
                }
            }
        }

        return Ok(SvgConversionCtx::Layers((
            palette,
            indexed_image,
            layers,
            options,
        )));
    }

    Err(Error::LayerGenerationFailure)
}
