use actix_web::{error, put, web, web::Json, Error, HttpRequest, HttpResponse};
use bytes::{Bytes, BytesMut};
use futures::future::{ok, Future};
use futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use svg_converter::svg_converter::svg_converted_str_from_base64_image;

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

#[put("/svg/conversion")]
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
    let res: Result<String, failure::Error> =
        svg_converted_str_from_base64_image(data.image_base64_data.clone());

    match res {
        Ok(svg_string) => Ok(
            HttpResponse::Ok().json(SvgConvertResponse::new(&data.image_file_name, &svg_string))
        ),
        Err(err_msg) => {
            Ok(HttpResponse::NotFound().json(format!("{}: {}", data.image_file_name, err_msg)))
        }
    }
}
