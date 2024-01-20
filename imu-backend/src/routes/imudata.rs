use crate::data_processing::{count_repetitions, filter_noise, get_imudata_result, get_processed_data, handle_lines};
use crate::errors::{ImuServerError, ServerResponseError};
use crate::helpers::files::process_raw_csv;
use crate::models::imudata::ImuDataResult;
use actix_multipart::Multipart;
use actix_web::web::{Json, Path};
use actix_web::{error::Error, get, post, HttpResponse, Responder};
use log::error;

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
    let processed_data = get_processed_data(&raw_data, 100).map_err(|e| {
        error!("Failed to process data: {:?}", e);
        ServerResponseError(ImuServerError::DataProcessing.into())
    })?;
    let repetitions: u32 = count_repetitions(&raw_data);
    let imudata = get_imudata_result(processed_data, repetitions, raw_data)?;
    Ok(Json(imudata))
}

#[post("/imudata_file/{mass_parameter}")]
async fn imudata_file(mass_parameter: Path<String>, payload: Multipart) -> Result<Json<ImuDataResult>, Error> {
    let mass = mass_parameter.parse::<u32>().map_err(|e| {
        error!("Failed to parse mass: {:?}", e);
        ServerResponseError(ImuServerError::InvalidInputData.into())
    })?;
    let mut raw_data = process_raw_csv(payload).await.map_err(|e| {
        error!("Failed to get raw data from input: {:?}", e);
        ServerResponseError(ImuServerError::FileNotFound.into())
    })?;
    let filtered_raw_data = filter_noise(&mut raw_data);
    let processed_data = get_processed_data(&filtered_raw_data, mass).map_err(|e| {
        error!("Failed to process raw data: {:?}", e);
        ServerResponseError(ImuServerError::DataProcessing.into())
    })?;
    let repetitions: u32 = count_repetitions(&filtered_raw_data);
    let imudata_result = get_imudata_result(processed_data, repetitions, filtered_raw_data).map_err(|e| {
        error!("Failed to summarize final results: {:?}", e);
        ServerResponseError(ImuServerError::DataProcessing.into())
    })?;
    Ok(Json(imudata_result))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
