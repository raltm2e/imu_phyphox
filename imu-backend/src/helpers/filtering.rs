use crate::models::imudata::RawData;

pub fn moving_average(raw_data: &[RawData], window_size: usize) -> Vec<f32> {
    let mut result = Vec::new();
    for i in window_size..raw_data.len() {
        let window = &raw_data[i-window_size..i];
        let sum: f32 = window.iter().map(|data| data.linear_acceleration_z).sum();
        let avg = sum / window_size as f32;
        result.push(avg);
    }
    result
}
