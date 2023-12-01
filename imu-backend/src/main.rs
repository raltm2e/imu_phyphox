use std::path::Path;
use imu_backend::data_processing::{get_processed_data, get_imudata_result};

fn main() {
    let filename = "../Acceleration without g 2023-03-26 13-07-06/Raw Data.csv";
    let processed_data_result = get_processed_data(Path::new(filename), 100);
    let processed_data = processed_data_result.map_err(|e| println!("{:?}", e));
    let imudata_result = get_imudata_result(processed_data.unwrap());
    println!("Reps: {:?}", imudata_result.unwrap().repetitions);
}
