use actix_web::{get, post, error::Error, App, HttpResponse, HttpServer, Responder};
use imu_backend::data_processing::{get_processed_data, get_imudata_result, handle_lines};

#[post("/hue")]
async fn hue(
    req_body: String,
) -> Result<HttpResponse, Error> {
    // let filename = "../Acceleration without g 2023-03-26 13-07-06/Raw Data.csv";
    // let raw_data = get_raw_data_from_file_path(Path::new(filename)).unwrap();
    let raw_data = handle_lines(req_body.lines().map(|l| l.to_string()).collect::<Vec<String>>()).unwrap();
    let processed_data_result = get_processed_data(raw_data, 100);
    let processed_data = processed_data_result.map_err(|e| println!("{:?}", e));
    let imudata_result = get_imudata_result(processed_data.unwrap());
    Ok(HttpResponse::Ok().body(imudata_result.unwrap().spent_energy.to_string()))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hue)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
