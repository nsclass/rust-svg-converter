
#[derive(Serialize, Deserialize)]
struct HealthStatus {
    status: String
}

pub fn health(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(HealthStatus { status: "UP".to_string()})
}
