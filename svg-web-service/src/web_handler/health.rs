use actix_web::{
    get, HttpRequest, HttpResponse,
};

#[derive(Serialize, Deserialize)]
struct HealthStatus {
    status: String
}

#[get("/health")]
pub fn health(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(HealthStatus { status: "UP".to_string()})
}
