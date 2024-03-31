use actix_web::{App, HttpServer};
use imu_backend::routes::imudata::{health, imudata_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health).service(imudata_file))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
