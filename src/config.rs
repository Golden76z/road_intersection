use crate::simulation::Vehicle;
use std::collections::VecDeque;

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

// Vec storing all the spawning vehicle lanes
pub const TOP_LEFT: VecDeque<Vehicle> = VecDeque::new();
pub const TOP_RIGHT: VecDeque<Vehicle> = VecDeque::new();
pub const BOTTOM_LEFT: VecDeque<Vehicle> = VecDeque::new();
pub const BOTTOM_RIGHT: VecDeque<Vehicle> = VecDeque::new();

// Intersection vec, storing transitioning vehicles
pub const INTERSECTION: VecDeque<Vehicle> = VecDeque::new();
