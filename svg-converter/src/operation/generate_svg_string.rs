use crate::{Error, ImagePathTrace, SvgConversionCtx};
use std::collections::BTreeMap;

pub fn generate_svg_string(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, trace_layers, options)) = ctx.into_image_path_trace() {
        let width = indexed_image.width - 2;
        let height = indexed_image.height - 2;
        let w = width * options.scale as usize;
        let h = height * options.scale as usize;
        let view_box_or_viewport = if options.view_box != 0. {
            format!("viewBox=\"0 0 {} {}\" ", w, h)
        } else {
            format!("width=\"{}\" height=\"{}\" ", w, h)
        };

        let mut svg_string = format!(
            "<svg {}version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" ",
            view_box_or_viewport
        );
        if options.show_description {
            svg_string.push_str("desc=\"Created by an image SVG converter\"");
        }
        svg_string.push_str(">");
        struct ZIndexValue(usize, usize);
        let mut z_index: BTreeMap<String, ZIndexValue> = BTreeMap::new();
        // map.entry(key).or_insert_with(|| default)

        // creating Z-index
        // Layer loop
        for layer_idx in 0..trace_layers.len() {
            // Path loop
            for path_count in 0..trace_layers.index_at(layer_idx).len() {
                // Label (Z-index key) is the startPoint of the path, linearized
                let label = (trace_layers.value_at(layer_idx, path_count, 0, 2) * w as f64)
                    + trace_layers.value_at(layer_idx, path_count, 0, 1);
                let key = format!("{:06.2}", label);
                z_index
                    .entry(key)
                    .or_insert_with(|| ZIndexValue(layer_idx, path_count));
                // Creating new list if required
            }
        }

        // Sorting Z-index is not required, TreeMap is sorted automatically

        // Drawing
        // Z-index loop
        for (_key, value) in z_index {
            let mut cur_desc = "".to_string();
            if options.show_description {
                cur_desc = format!("desc=\"l {} p {}\" ", value.0, value.1);
            }
            generate_svg_path_from_trace_paths(
                &mut svg_string,
                cur_desc,
                trace_layers.index_at(value.0).index_at(value.1),
                to_svg_color_string(palette[value.0]),
                &options,
            );
        }

        // SVG End
        svg_string.push_str("</svg>");
        return Ok(SvgConversionCtx::SvgString(svg_string));
    }
    Err(Error::FailureGeneratingSvgString)
}

fn generate_svg_path_from_trace_paths(
    sb: &mut String,
    desc: String,
    segments: &ImagePathTrace,
    color_str: String,
    options: &crate::ImageConvertOptions,
) {
    let scale = options.scale as f64;
    let lcpr = options.l_cpr;
    let qcpr = options.q_cpr;
    let round_coords = options.round_coords;

    // Path
    let path_str = format!(
        "<path {}{}d=\"M {} {} ",
        desc,
        color_str,
        segments.value_at(0, 1) * scale,
        segments.value_at(0, 2) * scale
    );
    sb.push_str(&path_str);

    if round_coords == -1. {
        segments.values().for_each(|segment| {
            if segment[0] == 1.0 {
                let svg_str = format!("L {} {} ", segment[3] * scale, segment[4] * scale);
                sb.push_str(&svg_str);
            } else {
                let svg_str = format!(
                    "Q {} {} {} {} ",
                    segment[3] * scale,
                    segment[4] * scale,
                    segment[5] * scale,
                    segment[6] * scale
                );
                sb.push_str(&svg_str);
            }
        });
    } else {
        segments.values().for_each(|segment| {
            if segment[0] == 1.0 {
                let svg_str = format!(
                    "L {} {} ",
                    round_to_decimal((segment[3] * scale) as f32, round_coords),
                    round_to_decimal((segment[4] * scale) as f32, round_coords)
                );
                sb.push_str(&svg_str);
            } else {
                let svg_str = format!(
                    "Q {} {} {} {} ",
                    round_to_decimal((segment[3] * scale) as f32, round_coords),
                    round_to_decimal((segment[4] * scale) as f32, round_coords),
                    round_to_decimal((segment[5] * scale) as f32, round_coords),
                    round_to_decimal((segment[6] * scale) as f32, round_coords)
                );
                sb.push_str(&svg_str);
            }
        });
    } // End of roundCoords check

    sb.push_str("Z\" />");

    // Rendering control points
    segments.values().for_each(|segment| {
        if (lcpr > 0.) && (segment[0] == 1.0) {
            let svg_str = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{} \" fill=\"white\" stroke-width=\"{} \" stroke=\"black\" />",
                segment[3] * scale, segment[4] * scale, lcpr, lcpr * 0.2);

            sb.push_str(&svg_str);
        }
        if (qcpr > 0.) && (segment[0] == 2.0) {
            let svg_str = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{} \" fill=\"cyan\" stroke-width=\"{} \" stroke=\"black\" />",
                segment[3] * scale, segment[4] * scale, qcpr, qcpr * 0.2);
            sb.push_str(&svg_str);

            let svg_str = format!("<circle cx=\"{}\" cy=\"{}\" r=\"{} \" fill=\"white\" stroke-width=\"{} \" stroke=\"black\" />",
                segment[5] * scale, segment[6] * scale, qcpr, qcpr * 0.2);
            sb.push_str(&svg_str);

            let svg_str = format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke-width=\"{}\" stroke=\"cyan\" />", 
                segment[1] * scale, segment[2] * scale, segment[3] * scale,segment[4] * scale,qcpr * 0.2);
            sb.push_str(&svg_str);

            let svg_str = format!("<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke-width=\"{}\" stroke=\"cyan\" />", 
                segment[3] * scale, segment[4] * scale, segment[5] * scale,segment[6] * scale,qcpr * 0.2);
            sb.push_str(&svg_str);
        }
    });
}

fn round_to_decimal(val: f32, places: f32) -> f32 {
    (val * f32::powf(10., places)).round() / f32::powf(10., places)
}

fn to_svg_color_string(clr: exoquant::Color) -> String {
    format!(
        "fill=\"rgb({},{},{})\" stroke=\"rgb({},{},{})\" stroke-width=\"1\" opacity=\"{}\" ",
        clr.r,
        clr.g,
        clr.b,
        clr.r,
        clr.g,
        clr.b,
        clr.a as f32 / 255.0
    )
}
