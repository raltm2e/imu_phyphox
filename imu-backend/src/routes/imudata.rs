use crate::data_processing::{get_imudata_result, get_processed_data, handle_lines};
use crate::errors::{ImuServerError, ServerResponseError};
use actix_web::{error::Error, get, post, HttpResponse, Responder};
use actix_web::web::Json;
use log::error;
use crate::models::imudata::ImuDataResult;

#[post("/imudata")]
async fn imudata(req_body: String) -> Result<Json<ImuDataResult>, Error> {
    let raw_data = handle_lines(
        req_body
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>(),
    )
    .map_err(|e| {
        error!("Failed to get raw data from input: {:?}", e);
        ServerResponseError(ImuServerError::FileNotFound.into())
    })?;
    let processed_data = get_processed_data(raw_data, 100).map_err(|e| {
        error!("Failed to process data: {:?}", e);
        ServerResponseError(ImuServerError::DataProcessing.into())
    })?;
    let imudata = get_imudata_result(processed_data)?;
    Ok(Json(imudata))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
