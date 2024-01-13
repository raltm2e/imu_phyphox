use crate::constants::MOVING_AVG_WINDOW_SIZE;
use crate::errors::{ImuServerError, ServerResponseError};
use crate::models::imudata::{ImuDataResult, ProcessedData, RawData};
use actix_web::error::Error;
use find_peaks::PeakFinder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::vec::Vec;
use crate::helpers::filtering::moving_average;

pub fn filter_noise(raw_data: &mut Vec<RawData>) -> Vec<RawData> {
    let smoothed_data = moving_average(raw_data, MOVING_AVG_WINDOW_SIZE);
    for (i, data) in raw_data.iter_mut().enumerate().skip(MOVING_AVG_WINDOW_SIZE) {
        data.linear_acceleration_z = smoothed_data[i - MOVING_AVG_WINDOW_SIZE];
    }
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

pub fn count_repetitions(raw_data: &[RawData]) -> u32 {
    let single_column: Vec<f32> = raw_data
        .iter()
        .map(|p| p.linear_acceleration_z)
        .collect();
    let mut fp = PeakFinder::new(single_column.as_ref());
    fp.with_min_prominence(6.5);
    fp.with_min_height(0.);
    let peaks = fp.find_peaks();
    peaks.len() as u32
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
    let mut previous_time = 0.0;
    let mut previous_velocity = 0.0;
    let mut total_distance = 0.0;
    let mut total_energy = 0.0;
    let mut processed_data: Vec<ProcessedData> = vec![];
    for raw_data_row in raw_data.iter() {
        let timestep = raw_data_row.time - previous_time;
        let velocity = get_velocity(
            raw_data_row.linear_acceleration_z,
            previous_velocity,
            timestep,
        );
        let distance_step = get_distance(velocity, timestep).abs();
        total_distance += distance_step;
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
            energy: total_energy,
            velocity,
        };
        processed_data.push(processed_data_row);
    }
    Ok(processed_data)
}

pub fn get_imudata_result(
    processed_data: Vec<ProcessedData>,
    repetitions: u32,
    raw_data: Vec<RawData>,
) -> Result<ImuDataResult, Error> {
    let last_row = match processed_data.last() {
        Some(data) => data,
        None => return Err(ServerResponseError(ImuServerError::DataProcessing.into()).into()),
    };
    let imu_data_result = ImuDataResult {
        repetitions,
        spent_time: last_row.time,
        total_distance: last_row.distance,
        spent_energy: last_row.energy,
        raw_data,
        processed_data,
    };
    Ok(imu_data_result)
}
