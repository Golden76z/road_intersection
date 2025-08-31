use crate::simulation::Vehicle;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// Grid constants for canva base drawings
pub const CASE_SIZE: u32 = CANVA_WIDTH / 20;
pub const GRID_WIDTH: u32 = CANVA_WIDTH / CASE_SIZE;
pub const GRID_HEIGHT: u32 = CANVA_HEIGHT / CASE_SIZE;
pub const GRID_WIDTH_PIXELS: u32 = GRID_WIDTH * CASE_SIZE;
pub const GRID_HEIGHT_PIXELS: u32 = GRID_HEIGHT * CASE_SIZE;

// Constants for the display settings
pub const CANVA_WIDTH: u32 = 1000;
pub const CANVA_HEIGHT: u32 = 1000;
pub const VEHICLE_WIDTH: u32 = CANVA_WIDTH / 20;
pub const VEHICLE_HEIGHT: u32 = CANVA_HEIGHT / 20;
// pub const TRAFFIC_LIGHT_WIDTH: u32 = CANVA_WIDTH / 20;
// pub const TRAFFIC_LIGHT_HEIGHT: u32 = CANVA_HEIGHT / 20;
pub const SAFE_DISTANCE: u32 = VEHICLE_WIDTH;
pub const VEHICLE_PER_LANE: u32 =
    (CANVA_WIDTH / 2 - VEHICLE_WIDTH) / (VEHICLE_WIDTH + SAFE_DISTANCE);
pub const VEHICLE_SPEED: u16 = 200;

// Starting and Ending positions of the Vehicles
pub const BOTTOM_SPAWN: (f32, f32) = (((CANVA_WIDTH / 2) + (VEHICLE_WIDTH / 2)) as f32, 1100.0);
pub const BOTTOM_DESTINATION: (f32, f32) =
    (((CANVA_WIDTH / 2) + (VEHICLE_WIDTH / 2)) as f32, -100.0);

pub const UP_SPAWN: (f32, f32) = (((CANVA_WIDTH / 2) - (VEHICLE_WIDTH / 2)) as f32, -100.0);
pub const UP_DESTINATION: (f32, f32) = (((CANVA_WIDTH / 2) - (VEHICLE_WIDTH / 2)) as f32, 1100.0);

pub const LEFT_SPAWN: (f32, f32) = (-100.0, ((CANVA_HEIGHT / 2) + (VEHICLE_HEIGHT / 2)) as f32);
pub const LEFT_DESTINATION: (f32, f32) =
    (1100.0, ((CANVA_HEIGHT / 2) + (VEHICLE_HEIGHT / 2)) as f32);

pub const RIGHT_SPAWN: (f32, f32) = (1100.0, ((CANVA_HEIGHT / 2) - (VEHICLE_HEIGHT / 2)) as f32);
pub const RIGHT_DESTINATION: (f32, f32) =
    (-100.0, ((CANVA_HEIGHT / 2) - (VEHICLE_HEIGHT / 2)) as f32);

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
