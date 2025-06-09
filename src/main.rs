use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/vehicle/speed")]
async fn get_speed() -> impl Responder {
    // スタブ：ランダムな車速を返す（0〜120 km/h）
    let speed = rand::random::<f32>() * 120.0;
    HttpResponse::Ok().json(json!({ "speed_kph": speed }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚗 Vehicle API running at http://localhost:8080");
    HttpServer::new(|| App::new().service(get_speed))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
