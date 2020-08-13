use crate::{
    BatchInterpolation, Error, ImagePathTraceLayers, ImagePathTraceList, SvgConversionCtx,
};

pub fn image_path_tracing(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, batch_list, options)) = ctx.into_batch_interpolation() {
        let mut trace_path_layers = Vec::<ImagePathTraceList>::new();
        for idx in 0..batch_list.len() {
            let trace_layer = generate_trace_path_layer(
                batch_list.index_at(idx),
                options.l_threshold,
                options.q_threshold,
            );
            trace_path_layers.push(trace_layer);
        }

        let layers = ImagePathTraceLayers::new(trace_path_layers);

        return Ok(SvgConversionCtx::ImagePathTraceLayers((
            palette,
            indexed_image,
            layers,
            options,
        )));
    }

    Err(Error::ImagePathTracingFailure)
}

fn generate_trace_path_layer(
    batch: &BatchInterpolation,
    l_threshold: f32,
    q_threshold: f32,
) -> ImagePathTraceList {
    ImagePathTraceList::new(batch, l_threshold, q_threshold)
}
