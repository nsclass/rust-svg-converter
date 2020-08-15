use crate::{
    BatchInterpolation, Error, ImagePathTraceLayers, ImagePathTraceList, SvgConversionCtx,
};

pub fn image_path_tracing(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, batch_list, options)) = ctx.into_batch_interpolation() {
        let trace_path_layers = batch_list
            .values()
            .map(|batch| {
                let trace_layer =
                    generate_trace_path_layer(batch, options.l_threshold, options.q_threshold);
                trace_layer
            })
            .collect();

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
