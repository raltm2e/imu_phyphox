use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;
use crate::constants::{ACCELERATION_NOISE_THRESHOLD_NEGATIVE, ACCELERATION_NOISE_THRESHOLD_POSITIVE};


fn filter_noise(data: &Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut new_data = data.clone();
    let mut changes = LinkedList::new();
    for i in 1..new_data.len() {
        let change = (new_data[i].1 - new_data[i - 1].1) / (new_data[i].0 - new_data[i - 1].0);
        changes.push_back(change);
    }
    for (i, change) in changes.iter().enumerate() {
        if *change > ACCELERATION_NOISE_THRESHOLD_POSITIVE || *change < ACCELERATION_NOISE_THRESHOLD_NEGATIVE {
            new_data[i].1 = 0.0;
        }
    }
    new_data
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

fn get_energy_spent(mass: i32, distance: f32, acceleration: f32) -> f32 {
    // A = F*s
    // F = m*a
    // A = m*a*s
    mass as f32 * acceleration * distance
}

fn _process_data(file_path: &Path, mass: i32) -> Vec<(f32, f32, f32, f32)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut previous_time = 0.0;
    let mut previous_velocity = 0.0;
    let mut total_distance = 0.0;
    let mut total_energy = 0.0;
    let mut velocity_vec = vec![];
    let mut distance_vec = vec![];
    let mut energy_vec = vec![];
    let mut data = vec![];
    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let values: Vec<f32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let time = values[0];
        let acceleration = values[2];
        let timestep = time - previous_time;
        let velocity = get_velocity(acceleration, previous_velocity, timestep);
        velocity_vec.push((time, velocity));
        let distance_step = get_distance(velocity, timestep).abs();
        total_distance += distance_step;
        distance_vec.push((time, total_distance));
        let energy_step = get_energy_spent(mass, distance_step, acceleration.abs());
        total_energy += energy_step;
        energy_vec.push((time, total_energy));
        previous_velocity = velocity;
        previous_time = time;
        data.push((time, acceleration, velocity, total_distance, total_energy));
    }
    let filtered_data = filter_noise(&distance_vec);
    let mut processed_data = vec![];
    for (i, (time, _, _, distance, energy)) in data.iter().enumerate() {
        if filtered_data[i].1 != 0.0 {
            processed_data.push((*time, *distance, *energy, velocity_vec[i].1));
        }
    }
    processed_data
}
