use std::collections::HashMap;

const CANVA_WIDTH: u32 = 1000;
const CANVA_HEIGHT: u32 = 1000;
const VEHICLE_WIDTH: u32 = CANVA_WIDTH / 20;
const VEHICLE_HEIGHT: u32 = CANVA_HEIGHT / 20;
const TRAFFIC_LIGHT_WIDTH: u32 = CANVA_WIDTH / 20;
const TRAFFIC_LIGHT_HEIGHT: u32 = CANVA_HEIGHT / 20;
const SAFE_DISTANCE: u32 = VEHICLE_WIDTH;
const VEHICLE_PER_LANE: u32 = (CANVA_WIDTH - VEHICLE_WIDTH) / (VEHICLE_WIDTH + SAFE_DISTANCE);

const COLORS: [(&str, (u8, u8, u8)); 3] = [
    ("LEFT", (220, 220, 30)),
    ("UP", (30, 220, 220)),
    ("RIGHT", (220, 30, 220)),
];

pub struct Config {
    canva_width: u32,
    canva_height: u32,

    vehicle_width: u32,
    vehicle_height: u32,

    traffic_light_width: u32,
    traffic_light_height: u32,

    // road_width: u32,
    // road_height: u32,
    top_left: Vec<Vehicle>,
    top_right: Vec<Vehicle>,
    bottom_left: Vec<Vehicle>,
    bottom_right: Vec<Vehicle>,

    intersection: Vec<Vehicle>,
    // colors: [(str, u8)],
}

impl Config {
    pub fn new() -> Self {
        Config {
            canva_width: CANVA_WIDTH,
            canva_height: CANVA_HEIGHT,

            vehicle_width: VEHICLE_WIDTH,
            vehicle_height: VEHICLE_HEIGHT,

            traffic_light_width: TRAFFIC_LIGHT_WIDTH,
            traffic_light_height: TRAFFIC_LIGHT_HEIGHT,

            // Vec storing all the spawning vehicle lanes
            top_left: vec!["", VEHICLE_PER_LANE],
            top_right: vec!["", VEHICLE_PER_LANE],
            bottom_left: vec!["", VEHICLE_PER_LANE],
            bottom_right: vec!["", VEHICLE_PER_LANE],

            // Intersection vec, storing transitioning vehicles
            intersection: vec!["", 4],
            // Color struct storing all 3 colors representing the directions
            // Colors: COLORS,
        }
    }
}
