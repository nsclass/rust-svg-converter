use crate::{
    BatchInterpolation, Error, InterpolationBatchList, InterpolationNodeList, ScanPaths,
    SvgConversionCtx,
};

fn create_interpolation_node_list(scan_path: &ScanPaths) -> BatchInterpolation {
    let mut inter_node_list = Vec::<InterpolationNodeList>::new();
    for idx in 0..scan_path.len() {
        let node_list = InterpolationNodeList::new(scan_path.index_at(idx));
        inter_node_list.push(node_list);
    }
    assert!(inter_node_list.len() > 0);
    BatchInterpolation::new(inter_node_list)
}

pub fn generate_batch_interpolation_list(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, scan_paths, options)) = ctx.into_scan_paths() {
        let mut batch_list = Vec::<BatchInterpolation>::new();
        for idx in 0..scan_paths.len() {
            let scan_path = scan_paths.index_at(idx);

            // create InterpolationNodeList
            let batch = create_interpolation_node_list(scan_path);
            batch_list.push(batch);
        }

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
