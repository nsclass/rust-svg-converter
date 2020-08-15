// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::{Error, ScanPathList, ScanPaths, SvgConversionCtx};

pub fn generate_scan_paths(ctx: SvgConversionCtx) -> Result<SvgConversionCtx, Error> {
    if let Some((palette, indexed_image, mut layers, options)) = ctx.into_layers() {
        let mut scan_paths = Vec::<ScanPaths>::new();
        for idx in 0..layers.len() {
            let layer = &mut layers[idx];

            // create scan path
            let scan_path = ScanPaths::new(layer, options.path_omit);
            scan_paths.push(scan_path)
        }
        let scan_list = ScanPathList::new(scan_paths);
        return Ok(SvgConversionCtx::ScanPaths((
            palette,
            indexed_image,
            scan_list,
            options,
        )));
    }

    Err(Error::ScanPathGenerationFailure)
}
