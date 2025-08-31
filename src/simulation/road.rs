use std::collections::VecDeque;

use crate::{config::VEHICLE_PER_LANE, simulation::Vehicle};

// Monitoring of the trafic lights
pub fn traffic_check() {}

// Vehicle spawn check
pub fn can_spawn_vehicle(vec: &VecDeque<Vehicle>) -> bool {
    vec.len() < VEHICLE_PER_LANE as usize
}
