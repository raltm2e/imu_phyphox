use crate::data_processing::{
    count_repetitions, filter_noise, get_imudata_result, get_processed_data,
};
use crate::errors::{ImuServerError, ServerResponseError};
use crate::helpers::files::process_raw_csv;
use crate::models::imudata::ImuDataResult;
use actix_multipart::Multipart;
use actix_web::web::{Json, Path};
use actix_web::{error::Error, get, post, HttpResponse, Responder};
use log::error;

#[post("/imudata_file/{mass_parameter}&{noise_parameter}")]
async fn imudata_file(
    path_variables: Path<(String, String)>,
    payload: Multipart,
) -> Result<Json<ImuDataResult>, Error> {
    let mass_parameter = path_variables.0.clone();
    let noise = path_variables.1.clone();
    let mass = mass_parameter.parse::<u32>().map_err(|e| {
        error!("Failed to parse mass: {:?}", e);
        ServerResponseError(ImuServerError::InvalidInputData.into())
    })?;
    let mut raw_data = process_raw_csv(payload).await.map_err(|e| {
        error!("Failed to get raw data from input: {:?}", e);
        ServerResponseError(ImuServerError::FileNotFound.into())
    })?;
    let filtered_raw_data = filter_noise(&mut raw_data, noise);
    let processed_data = get_processed_data(&filtered_raw_data, mass).map_err(|e| {
        error!("Failed to process raw data: {:?}", e);
        ServerResponseError(ImuServerError::DataProcessing.into())
    })?;
    let repetitions: u32 = count_repetitions(&filtered_raw_data);
    let imudata_result = get_imudata_result(processed_data, repetitions, mass, filtered_raw_data)
        .map_err(|e| {
            error!("Failed to summarize final results: {:?}", e);
            ServerResponseError(ImuServerError::DataProcessing.into())
        })?;
    Ok(Json(imudata_result))
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
