// Copyright (c) 2020 Nam Seob Seo
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use actix_web::{HttpRequest, HttpResponse};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct HealthStatus {
    status: String,
}

pub async fn health(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(HealthStatus {
        status: "UP".to_string(),
    })
}
