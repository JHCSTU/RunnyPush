use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SunnyData {
    pub count: i32,
    pub average_speed: f32,
}

#[derive(Serialize, Deserialize)]
pub struct SunnyList {
    data: Vec<Sunny>,
}

#[derive(Serialize, Deserialize)]
pub struct Sunny {
    pub time: String,
    pub meters: String,
    pub speed: String,
    pub ok: bool,
}