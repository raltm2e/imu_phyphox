use crate::errors::{ImuServerError, ServerResponseError};
use actix_web::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawData {
    pub time: f32,
    pub linear_acceleration_x: f32,
    pub linear_acceleration_y: f32,
    pub linear_acceleration_z: f32,
    pub absolute_acceleration: f32,
}

impl TryFrom<Vec<f32>> for RawData {
    type Error = Error;

    fn try_from(values: Vec<f32>) -> Result<Self, Error> {
        if values.len() != 5 {
            return Err(ServerResponseError(ImuServerError::DataProcessing.into()).into());
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

pub trait EnergyConversion {
    fn joules_to_kcal(self) -> f32;
    fn kcal_to_joules(self) -> f32;
}

impl EnergyConversion for f32 {
    fn joules_to_kcal(self) -> f32 {
        self / 4184.0
    }

    fn kcal_to_joules(self) -> f32 {
        self * 4184.0
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProcessedData {
    pub time: f32,
    pub distance: f32,
    pub displacement: f32,
    pub energy: f32,
    pub velocity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImuDataResult {
    pub mass: u32,
    pub repetitions: u32,
    pub spent_time: f32,
    pub total_distance: f32,
    pub spent_energy: f32,
    pub raw_data: Vec<RawData>,
    pub processed_data: Vec<ProcessedData>,
}
