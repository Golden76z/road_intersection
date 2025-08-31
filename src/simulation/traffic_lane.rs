use crate::{
    config::{
        BOTTOM_SPAWN, BOTTOM_VECTOR, Direction, LEFT_SPAWN, LEFT_VECTOR, RIGHT_SPAWN, RIGHT_VECTOR,
        TOP_VECTOR, UP_SPAWN,
    },
    simulation::{Vehicle, VehicleSpawn, can_spawn_vehicle},
};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

// Thread-safe vehicle lanes using Arc<Mutex<VecDeque<Vehicle>>>
pub type VehicleLane = Arc<Mutex<VecDeque<Vehicle>>>;

pub struct TrafficLanes {
    pub left: VehicleLane,
    pub right: VehicleLane,
    pub bottom: VehicleLane,
    pub up: VehicleLane,
    vehicle_id_counter: Arc<Mutex<i32>>, // To assign unique IDs to vehicles
}

impl TrafficLanes {
    pub fn new() -> Self {
        Self {
            left: Arc::new(Mutex::new(VecDeque::new())),
            right: Arc::new(Mutex::new(VecDeque::new())),
            bottom: Arc::new(Mutex::new(VecDeque::new())),
            up: Arc::new(Mutex::new(VecDeque::new())),
            vehicle_id_counter: Arc::new(Mutex::new(0)),
        }
    }

    fn get_next_vehicle_id(&self) -> i32 {
        let mut counter = self.vehicle_id_counter.lock().unwrap();
        *counter += 1;
        *counter
    }

    pub fn spawn_vehicle(&self, spawn_point: &str) {
        match spawn_point {
            "up" => {
                let mut up_lane = self.up.lock().unwrap();
                if can_spawn_vehicle(&up_lane) {
                    up_lane.push_back(Vehicle::new(
                        self.get_next_vehicle_id(),
                        UP_SPAWN,
                        BOTTOM_VECTOR,
                        Direction::random(),
                        VehicleSpawn::North,
                    ));
                }
            }
            "down" => {
                let mut bottom_lane = self.bottom.lock().unwrap();
                if can_spawn_vehicle(&bottom_lane) {
                    bottom_lane.push_back(Vehicle::new(
                        self.get_next_vehicle_id(),
                        BOTTOM_SPAWN,
                        TOP_VECTOR,
                        Direction::random(),
                        VehicleSpawn::South,
                    ));
                }
            }
            "left" => {
                let mut left_lane = self.left.lock().unwrap();
                if can_spawn_vehicle(&left_lane) {
                    left_lane.push_back(Vehicle::new(
                        self.get_next_vehicle_id(),
                        LEFT_SPAWN,
                        RIGHT_VECTOR,
                        Direction::random(),
                        VehicleSpawn::West,
                    ));
                }
            }
            _ => {
                // "right"
                let mut right_lane = self.right.lock().unwrap();
                if can_spawn_vehicle(&right_lane) {
                    right_lane.push_back(Vehicle::new(
                        self.get_next_vehicle_id(),
                        RIGHT_SPAWN,
                        LEFT_VECTOR,
                        Direction::random(),
                        VehicleSpawn::East,
                    ));
                }
            }
        }
    }

    // Method to get total vehicle count across all lanes (useful for debugging)
    pub fn total_vehicle_count(&self) -> usize {
        let up_count = self.up.lock().unwrap().len();
        let bottom_count = self.bottom.lock().unwrap().len();
        let left_count = self.left.lock().unwrap().len();
        let right_count = self.right.lock().unwrap().len();

        up_count + bottom_count + left_count + right_count
    }

    // Method to get vehicle counts for each lane (useful for debugging)
    pub fn get_lane_counts(&self) -> (usize, usize, usize, usize) {
        let up_count = self.up.lock().unwrap().len();
        let bottom_count = self.bottom.lock().unwrap().len();
        let left_count = self.left.lock().unwrap().len();
        let right_count = self.right.lock().unwrap().len();

        (up_count, bottom_count, left_count, right_count)
    }
}
