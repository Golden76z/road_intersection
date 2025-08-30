use crate::simulation::Vehicle;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// Constants for the display settings
pub const CANVA_WIDTH: u32 = 1000;
pub const CANVA_HEIGHT: u32 = 1000;
pub const VEHICLE_WIDTH: u32 = CANVA_WIDTH / 20;
pub const VEHICLE_HEIGHT: u32 = CANVA_HEIGHT / 20;
pub const TRAFFIC_LIGHT_WIDTH: u32 = CANVA_WIDTH / 20;
pub const TRAFFIC_LIGHT_HEIGHT: u32 = CANVA_HEIGHT / 20;
pub const SAFE_DISTANCE: u32 = VEHICLE_WIDTH;
pub const VEHICLE_PER_LANE: u32 =
    (CANVA_WIDTH / 2 - VEHICLE_WIDTH) / (VEHICLE_WIDTH + SAFE_DISTANCE);
pub const VEHICLE_SPEED: u16 = 200;

// Starting and Ending positions of the Vehicles
pub const BOTTOM_SPAWN: (f32, f32) = (((CANVA_WIDTH / 2) + (VEHICLE_WIDTH / 2)) as f32, 1100.0);
pub const BOTTOM_DESTINATION: (f32, f32) =
    (((CANVA_WIDTH / 2) + (VEHICLE_WIDTH / 2)) as f32, -100.0);

// Colors depending on the direction
pub enum Direction {
    Left,
    Up,
    Right,
}
impl Direction {
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Direction::Left => (220, 220, 30),
            Direction::Up => (30, 220, 220),
            Direction::Right => (220, 30, 220),
        }
    }
}

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
}

// Vec storing all the spawning vehicle lanes
// pub static mut LEFT: VecDeque<Vehicle> = VecDeque::new();
// pub static mut RIGHT: VecDeque<Vehicle> = VecDeque::new();
// pub static mut BOTTOM: VecDeque<Vehicle> = VecDeque::new();
// pub static mut UP: VecDeque<Vehicle> = VecDeque::new();

// Intersection vec, storing transitioning vehicles
pub const INTERSECTION: VecDeque<Vehicle> = VecDeque::new();
