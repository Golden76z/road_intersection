use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use rand::Rng;

use crate::{
    config::{
        BOTTOM_DESTINATION, BOTTOM_SPAWN, LEFT_DESTINATION, LEFT_SPAWN, RIGHT_DESTINATION,
        RIGHT_SPAWN, UP_DESTINATION, UP_SPAWN,
    },
    simulation::{Vehicle, VehicleDirection, can_spawn_vehicle},
};

// Thread-safe vehicle lanes using Arc<Mutex<VecDeque<Vehicle>>>
pub type VehicleLane = Arc<Mutex<VecDeque<Vehicle>>>;

pub struct TrafficLanes {
    pub left: VehicleLane,
    pub right: VehicleLane,
    pub bottom: VehicleLane,
    pub up: VehicleLane,
}

impl TrafficLanes {
    pub fn new() -> Self {
        Self {
            left: Arc::new(Mutex::new(VecDeque::new())),
            right: Arc::new(Mutex::new(VecDeque::new())),
            bottom: Arc::new(Mutex::new(VecDeque::new())),
            up: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn_vehicle(&self, spawn_point: &str) {
        let direction: VehicleDirection;
        let mut rng = rand::rng();
        let rand_num = rng.random_range(0..3);

        match spawn_point {
            "up" => {
                match rand_num {
                    0 => direction = VehicleDirection::West,
                    1 => direction = VehicleDirection::South,
                    _ => direction = VehicleDirection::East,
                }

                let mut up_lane = self.up.lock().unwrap();
                if can_spawn_vehicle(&*up_lane) {
                    up_lane.push_back(Vehicle::new(1, UP_SPAWN, UP_DESTINATION, direction));
                }
            }
            "down" => {
                match rand_num {
                    0 => direction = VehicleDirection::West,
                    1 => direction = VehicleDirection::North,
                    _ => direction = VehicleDirection::East,
                }

                let mut bottom_lane = self.bottom.lock().unwrap();
                if can_spawn_vehicle(&*bottom_lane) {
                    bottom_lane.push_back(Vehicle::new(
                        1,
                        BOTTOM_SPAWN,
                        BOTTOM_DESTINATION,
                        direction,
                    ));
                }
            }
            "left" => {
                match rand_num {
                    0 => direction = VehicleDirection::North,
                    1 => direction = VehicleDirection::East,
                    _ => direction = VehicleDirection::South,
                }

                let mut left_lane = self.left.lock().unwrap();
                if can_spawn_vehicle(&*left_lane) {
                    left_lane.push_back(Vehicle::new(1, LEFT_SPAWN, LEFT_DESTINATION, direction));
                }
            }
            _ => {
                match rand_num {
                    0 => direction = VehicleDirection::South,
                    1 => direction = VehicleDirection::East,
                    _ => direction = VehicleDirection::North,
                }

                let mut right_lane = self.right.lock().unwrap();
                if can_spawn_vehicle(&*right_lane) {
                    right_lane.push_back(Vehicle::new(
                        1,
                        RIGHT_SPAWN,
                        RIGHT_DESTINATION,
                        direction,
                    ));
                }
            }
        }
    }
}

// Intersection vec, storing transitioning vehicles
pub const INTERSECTION: VecDeque<Vehicle> = VecDeque::new();
