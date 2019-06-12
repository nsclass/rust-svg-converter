use actix_web::{
    put, HttpRequest, HttpResponse, web::Json
};

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
            image_file_name : String::from(file_name),
            svg_string: String::from(svg_string)
        }
    }
}
#[put("/svg/conversion")]
pub fn svg_convert(req: HttpRequest, data: Json<SvgConvertRequest>) -> HttpResponse {

    let res = svg_converter::svg_converted_str_from_base64_image(data.image_base64_data.clone());

    match res {
        Ok(svg_string) => HttpResponse::Ok().json(SvgConvertResponse::new(&data.image_file_name, &svg_string)),
        Err(err_msg) => HttpResponse::NotFound().json(format!("{}: {}", data.image_file_name, err_msg)),
    }
}

