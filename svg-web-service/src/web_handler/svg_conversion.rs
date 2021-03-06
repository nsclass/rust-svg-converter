// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use bytes::BytesMut;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tracing::info;

use svg_converter::{svg_converted_str_from_base64_image, OperationProgressListener};

#[derive(Serialize, Deserialize)]
pub struct SvgConvertRequest {
    image_file_name: String,
    image_base64_data: String,
    number_of_colors: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SvgConvertResponse {
    image_file_name: String,
    svg_string: String,
}

impl SvgConvertResponse {
    pub fn new(file_name: &str, svg_string: &str) -> SvgConvertResponse {
        SvgConvertResponse {
            image_file_name: String::from(file_name),
            svg_string: String::from(svg_string),
        }
    }
}

struct ProgressListener {}

impl OperationProgressListener for ProgressListener {
    fn on_progress(&self, desc: &str, cur: usize, total: usize) {
        info!("{}, {}/{}", desc, cur, total);
    }
}

#[tracing::instrument(name = "svg_convert", skip(_req, payload))]
pub async fn svg_convert(
    _req: HttpRequest,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    // payload is a stream of Bytes objects
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        // if (body.len() + chunk.len()) > MAX_SIZE {
        //     return Err(error::ErrorBadRequest("overflow"));
        // }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let data = serde_json::from_slice::<SvgConvertRequest>(&body)?;
    let res: Result<String, svg_converter::Error> =
        svg_converted_str_from_base64_image(data.image_base64_data.clone(), &ProgressListener {});

    return res
        .map(|svg_str| {
            HttpResponse::Ok().json(SvgConvertResponse::new(&data.image_file_name, &svg_str))
        })
        .map_err(|err| error::ErrorBadRequest(err));
}
