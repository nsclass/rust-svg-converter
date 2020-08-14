#[derive(Debug)]
pub struct ImageConvertOptions {
    pub l_threshold: f32,
    pub q_threshold: f32,
    pub path_omit: u32,
    pub color_sampling: bool,
    pub number_of_colors: u32,
    pub min_color_ratio: f32,
    pub color_quant_cycles: u32,
    pub scale: f32,
    pub simplify_tolerance: f32,
    pub round_coords: f32,
    pub l_cpr: f32,
    pub q_cpr: f32,
    pub show_description: bool,
    pub view_box: f32,
    pub blur_radius: f32,
    pub blur_delta: f32,
}

impl Default for ImageConvertOptions {
    fn default() -> ImageConvertOptions {
        ImageConvertOptions {
            l_threshold: 1.0,
            q_threshold: 1.0,
            path_omit: 8,
            color_sampling: true,
            number_of_colors: 16,
            min_color_ratio: 0.02,
            color_quant_cycles: 3,
            scale: 1.0,
            simplify_tolerance: 0.0,
            round_coords: 1.0,
            l_cpr: 0.0,
            q_cpr: 0.0,
            show_description: false,
            view_box: 0.0,
            blur_radius: 0.0,
            blur_delta: 20.0,
        }
    }
}
