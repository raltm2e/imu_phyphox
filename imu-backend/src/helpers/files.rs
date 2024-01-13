use crate::constants::TEMP_FILE_PATH;
use crate::data_processing::get_raw_data_from_file_path;
use crate::models::imudata::RawData;
use actix_multipart::Multipart;
use actix_web::error::Error;
use futures::{StreamExt, TryStreamExt};
use log::{debug, error};
use sanitize_filename::sanitize;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::PathBuf;
use crate::errors::{ImuServerError, ServerResponseError};

pub async fn process_raw_csv(mut payload: Multipart) -> Result<Vec<RawData>, Error> {
    let mut raw_data = vec![];
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type.get_filename();
        debug!("Filename: {:?}", filename);
        if let Some(filename) = filename {
            let filepath = PathBuf::from(format!("{}{}", TEMP_FILE_PATH, sanitize(filename)));
            if let Some(file_extension) = filepath.extension() {
                if file_extension != "csv" {
                    error!("File extension is not CSV");
                    return Err(ServerResponseError(ImuServerError::FileNotFound.into()).into());
                }
            }
            let mut file = File::create(&filepath)?;
            debug!("Created file");
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file.write_all(&data)?;
            }
            raw_data = get_raw_data_from_file_path(&filepath)?;
            remove_file(filepath)?;
        }
    }
    Ok(raw_data)
}
