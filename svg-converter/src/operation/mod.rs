// include!("palette_generation.rs");
// include!("color_quantization.rs");
mod generate_layer_edge_detection;
mod generate_palette_quantization;
mod generate_paths;

pub use generate_layer_edge_detection::*;
pub use generate_palette_quantization::*;
pub use generate_paths::*;
