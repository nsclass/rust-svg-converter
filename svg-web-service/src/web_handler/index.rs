// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use actix_files::NamedFile;
use std::path::PathBuf;

pub async fn single_page_app() -> actix_web::Result<NamedFile> {
    let path: PathBuf = PathBuf::from("./react-ui/build/index.html");
    Ok(NamedFile::open(path)?)
}
