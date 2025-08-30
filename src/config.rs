use crate::simulation::Vehicle;

pub const CANVA_WIDTH: u32 = 1000;
pub const CANVA_HEIGHT: u32 = 1000;
pub const VEHICLE_WIDTH: u32 = CANVA_WIDTH / 20;
pub const VEHICLE_HEIGHT: u32 = CANVA_HEIGHT / 20;
pub const TRAFFIC_LIGHT_WIDTH: u32 = CANVA_WIDTH / 20;
pub const TRAFFIC_LIGHT_HEIGHT: u32 = CANVA_HEIGHT / 20;
pub const SAFE_DISTANCE: u32 = VEHICLE_WIDTH;
pub const VEHICLE_PER_LANE: u32 = (CANVA_WIDTH - VEHICLE_WIDTH) / (VEHICLE_WIDTH + SAFE_DISTANCE);

pub const COLORS: [(&str, (u8, u8, u8)); 3] = [
    ("LEFT", (220, 220, 30)),
    ("UP", (30, 220, 220)),
    ("RIGHT", (220, 30, 220)),
];

pub struct Config {
    pub canva_width: u32,
    pub canva_height: u32,

    pub vehicle_width: u32,
    pub vehicle_height: u32,

    pub traffic_light_width: u32,
    pub traffic_light_height: u32,

    // road_width: u32,
    // road_height: u32,
    pub top_left: Vec<Vehicle>,
    pub top_right: Vec<Vehicle>,
    pub bottom_left: Vec<Vehicle>,
    pub bottom_right: Vec<Vehicle>,

    pub intersection: Vec<Vehicle>,
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
            top_left: Vec::with_capacity(VEHICLE_PER_LANE as usize),
            top_right: Vec::with_capacity(VEHICLE_PER_LANE as usize),
            bottom_left: Vec::with_capacity(VEHICLE_PER_LANE as usize),
            bottom_right: Vec::with_capacity(VEHICLE_PER_LANE as usize),

            // Intersection vec, storing transitioning vehicles
            intersection: Vec::with_capacity(VEHICLE_PER_LANE as usize),
            // Color struct storing all 3 colors representing the directions
            // Colors: COLORS,
        }
    }
}
