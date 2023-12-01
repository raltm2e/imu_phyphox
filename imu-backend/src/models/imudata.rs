use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RawData {
    pub time: f32,
    pub linear_acceleration_x: f32,
    pub linear_acceleration_y: f32,
    pub linear_acceleration_z: f32,
    pub absolute_acceleration: f32,
}

impl TryFrom<Vec<f32>> for RawData {
    type Error = &'static str;

    fn try_from(values: Vec<f32>) -> Result<Self, Self::Error> {
        if values.len() != 5 {
            return Err("Invalid number of values");
        }
        Ok(RawData {
            time: values[0],
            linear_acceleration_x: values[1],
            linear_acceleration_y: values[2],
            linear_acceleration_z: values[3],
            absolute_acceleration: values[4],
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProcessedData {
    pub time: f32,
    pub distance: f32,
    pub energy: f32,
    pub velocity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImuDataResult {
    pub processed_data: Vec<ProcessedData>,
    pub repetitions: u32,
    pub spent_time: f32,
    pub total_distance: f32,
    pub spent_energy: f32,
}
