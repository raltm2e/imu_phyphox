use actix_web::{App, HttpServer};
use imu_backend::routes::imudata::{hello, imudata, imudata_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(imudata)
            .service(imudata_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
