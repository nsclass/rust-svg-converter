

pub fn health(_req: &HttpRequest) -> impl Responder {
    r#"{ "status" : "UP" }"#.to_string()
}
