use std::path::Path;
use imu_backend::data_processing;

fn main() {
    let filename = "../Acceleration without g 2023-03-26 13-07-06/Raw Data.csv";
    let processed_data_result = data_processing::process_data(Path::new(filename), 100);
    let processed_data = processed_data_result.map_err(|e| println!("{:?}", e));
    println!("{:?}", processed_data.unwrap().first());
}
