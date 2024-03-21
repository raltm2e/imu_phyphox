use crate::constants::{END_TIME_CUTOFF, MOVING_AVG_WINDOW_SIZE_HIGH, MOVING_AVG_WINDOW_SIZE_LOW, MOVING_AVG_WINDOW_SIZE_MEDIUM, START_TIME_CUTOFF};
use crate::errors::{ImuServerError, ServerResponseError};
use crate::helpers::filtering::{moving_average, Noise};
use crate::models::imudata::{EnergyConversion, ImuDataResult, ProcessedData, RawData};
use actix_web::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::vec::Vec;

pub fn filter_noise(raw_data: &mut Vec<RawData>, noise: String) -> Vec<RawData> {
    let noise_enum: Noise = noise.into();
    let window_size = match noise_enum {
        Noise::Low => MOVING_AVG_WINDOW_SIZE_LOW,
        Noise::Medium => MOVING_AVG_WINDOW_SIZE_MEDIUM,
        Noise::High => MOVING_AVG_WINDOW_SIZE_HIGH,
    };
    let smoothed_data = moving_average(raw_data, window_size);
    for (i, data) in raw_data.iter_mut().enumerate().skip(window_size) {
        data.linear_acceleration_z = smoothed_data[i - window_size];
    }
    let max_time = raw_data.iter().map(|data| data.time).fold(f32::MIN, f32::max);
    let end_time_cutoff = max_time - END_TIME_CUTOFF;
    raw_data.retain(|data| data.time >= START_TIME_CUTOFF && data.time <= end_time_cutoff);
    raw_data.clone()
}

fn get_velocity(acceleration: f32, v0: f32, delta_t: f32) -> f32 {
    // a = (V1-V0) / t
    // V1 = a*t + V0
    acceleration * delta_t + v0
}

fn get_distance(delta_v: f32, delta_t: f32) -> f32 {
    // V = s/t
    // s = V*t
    delta_v * delta_t
}

fn get_energy_spent(mass: u32, distance: f32, acceleration: f32) -> f32 {
    // A = F*s
    // F = m*a
    // A = m*a*s
    mass as f32 * acceleration * distance
}

fn get_upward_motion_indices(displacement_vec: &[f32]) -> (Vec<usize>, Vec<usize>) {
    let mut start_indices = Vec::new();
    let mut end_indices = Vec::new();
    for i in 1..displacement_vec.len() - 1 {
        if displacement_vec[i - 1] < displacement_vec[i]
            && displacement_vec[i] >= displacement_vec[i + 1]
        {
            end_indices.push(i);
        }
        if displacement_vec[i - 1] >= displacement_vec[i]
            && displacement_vec[i] < displacement_vec[i + 1]
        {
            start_indices.push(i);
        }
    }
    (start_indices, end_indices)
}

pub fn handle_lines(lines: Vec<String>) -> Result<Vec<RawData>, Error> {
    let mut raw_data = vec![];
    for line in lines.iter().skip(1) {
        let values: Vec<f32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let raw_data_row: RawData = RawData::try_from(values)?;
        raw_data.push(raw_data_row);
    }
    Ok(raw_data)
}

pub fn get_raw_data_from_file_path(file_path: &PathBuf) -> Result<Vec<RawData>, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    handle_lines(
        reader
            .lines()
            .skip(1)
            .map(|l| l.unwrap())
            .collect::<Vec<String>>(),
    )
}

pub fn get_processed_data(raw_data: &[RawData], mass: u32) -> Result<Vec<ProcessedData>, Error> {
    let mut previous_time = 4.5;
    let mut previous_velocity = 0.0;
    let mut total_distance = 0.0;
    let mut current_distance = 0.0;
    let mut total_energy = 0.0;
    let mut processed_data: Vec<ProcessedData> = vec![];
    for raw_data_row in raw_data.iter() {
        let timestep = raw_data_row.time - previous_time;
        let velocity = get_velocity(
            raw_data_row.linear_acceleration_z,
            previous_velocity,
            timestep,
        );
        let distance_step = get_distance(velocity, timestep);
        total_distance += distance_step.abs();
        current_distance += distance_step;
        let energy_step = get_energy_spent(
            mass,
            distance_step,
            raw_data_row.linear_acceleration_z.abs(),
        );
        total_energy += energy_step;
        previous_velocity = velocity;
        previous_time = raw_data_row.time;
        let processed_data_row = ProcessedData {
            time: raw_data_row.time,
            distance: total_distance,
            displacement: current_distance,
            energy: total_energy,
            velocity,
        };
        processed_data.push(processed_data_row);
    }
    Ok(processed_data)
}

pub fn get_imudata_result(
    processed_data: Vec<ProcessedData>,
    mass: u32,
    raw_data: Vec<RawData>,
) -> Result<ImuDataResult, Error> {
    let last_row = match processed_data.last() {
        Some(data) => data,
        None => return Err(ServerResponseError(ImuServerError::DataProcessing.into()).into()),
    };

    let (start_indices, end_indices) = get_upward_motion_indices(
        &processed_data
            .iter()
            .map(|p| p.displacement)
            .collect::<Vec<f32>>(),
    );
    let energy_per_repetition: Vec<f32> = start_indices
        .iter()
        .zip(end_indices.iter())
        .map(|(start, end)| processed_data[*end].energy - processed_data[*start].energy)
        .collect();
    let total_energy_from_reps = energy_per_repetition.iter().sum::<f32>();
    let repetition_times: Vec<f32> = start_indices
        .iter()
        .zip(end_indices.iter())
        .map(|(start, end)| processed_data[*end].time - processed_data[*start].time)
        .collect();

    let imu_data_result = ImuDataResult {
        mass,
        repetitions: repetition_times.len() as u32,
        spent_time: last_row.time,
        total_distance: last_row.distance,
        spent_energy: total_energy_from_reps.joules_to_kcal(),
        raw_data,
        processed_data,
    };
    Ok(imu_data_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_velocity() {
        let acceleration = 2.0;
        let v0 = 0.0;
        let delta_t = 1.0;
        let expected_velocity = 2.0;
        assert_eq!(get_velocity(acceleration, v0, delta_t), expected_velocity);
    }

    #[test]
    fn test_get_distance() {
        let delta_v = 2.0;
        let delta_t = 4.0;
        let expected_distance = 8.0;
        assert_eq!(get_distance(delta_v, delta_t), expected_distance);
    }

    #[test]
    fn test_get_energy_spent() {
        let mass = 10;
        let distance = 2.0;
        let acceleration = 2.0;
        let expected_energy = 40.0;
        assert_eq!(
            get_energy_spent(mass, distance, acceleration),
            expected_energy
        );
    }
}
