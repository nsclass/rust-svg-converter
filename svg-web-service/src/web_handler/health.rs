use serde_derive::{Deserialize, Serialize};

use actix_web::{get, HttpRequest, HttpResponse};

#[derive(Serialize, Deserialize)]
struct HealthStatus {
    status: String,
}

#[get("/health")]
pub async fn health(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(HealthStatus {
        status: "UP".to_string(),
    })
}
