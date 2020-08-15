use crate::{
    BatchInterpolation, Error, InterpolationBatchList, InterpolationNodeList, ScanPaths,
    SvgConversionCtx,
};
use rayon::prelude::*;

fn create_interpolation_node_list(scan_path: &ScanPaths) -> BatchInterpolation {
    let inter_node_list: Vec<InterpolationNodeList> = scan_path
        .par_values()
        .map(|path| InterpolationNodeList::new(path))
        .collect();
    assert!(inter_node_list.len() > 0);
    BatchInterpolation::new(inter_node_list)
}

pub fn generate_batch_interpolation_list(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, scan_paths, options)) = ctx.into_scan_paths() {
        let batch_list = scan_paths
            .par_values()
            .map(|scan_path| create_interpolation_node_list(scan_path))
            .collect();

        let inter_batch_list = InterpolationBatchList::new(batch_list);
        return Ok(SvgConversionCtx::BatchInterpolation((
            palette,
            indexed_image,
            inter_batch_list,
            options,
        )));
    }
    Err(Error::BatchInterpolationGenerationFailure)
}
