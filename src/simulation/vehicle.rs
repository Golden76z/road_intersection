use crate::config::{
    // Add destination constants
    BOTTOM_DESTINATION,
    CASE_SIZE,
    Direction,
    EAST_LIGHT,
    LEFT_DESTINATION,
    NORTH_LIGHT,
    RIGHT_DESTINATION,
    SAFE_DISTANCE,
    SOUTH_LIGHT,
    UP_DESTINATION,
    VEHICLE_HEIGHT,
    VEHICLE_SPEED,
    VEHICLE_WIDTH,
    WEST_LIGHT,
};
use crate::simulation::TrafficLight;
use sdl2;
use sdl2::rect::Rect;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleSpawn {
    North,
    South,
    East,
    West,
}

impl VehicleSpawn {
    pub fn as_str(&self) -> &str {
        match self {
            VehicleSpawn::North => "North",
            VehicleSpawn::South => "South",
            VehicleSpawn::East => "East",
            VehicleSpawn::West => "West",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: i32,
    pub position: (f32, f32),
    pub vector: (f32, f32),
    pub speed: u16,
    pub spawn: VehicleSpawn,
    pub direction: Direction,
    pub shape: sdl2::rect::Rect,
    pub as_turned: bool,
}

impl<'a> Vehicle {
    pub fn new(
        id: i32,
        position: (f32, f32),
        vector: (f32, f32),
        direction: Direction,
        spawn: VehicleSpawn,
    ) -> Self {
        let mut vehicle = Vehicle {
            id,
            position,
            vector,
            speed: VEHICLE_SPEED,
            direction,
            spawn,
            shape: Rect::new(
                position.0 as i32 - (VEHICLE_WIDTH / 2) as i32,
                position.1 as i32 - (VEHICLE_HEIGHT / 2) as i32,
                VEHICLE_WIDTH,
                VEHICLE_HEIGHT,
            ),
            as_turned: false,
        };
        vehicle.accelerate();
        vehicle
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.direction.color());
        let _ = canvas.fill_rect(self.shape);
    }

    // Check if vehicle has reached its destination
    pub fn has_reached_destination(&self) -> bool {
        let destination = self.get_destination();
        let distance = ((self.position.0 - destination.0).powi(2)
            + (self.position.1 - destination.1).powi(2))
        .sqrt();

        // Consider destination reached if within 50 pixels
        distance < 50.0
    }

    // Get the destination based on spawn point and direction
    fn get_destination(&self) -> (f32, f32) {
        match self.spawn {
            VehicleSpawn::North => match self.direction {
                Direction::Up => UP_DESTINATION,
                Direction::Left => LEFT_DESTINATION,
                Direction::Right => RIGHT_DESTINATION,
            },
            VehicleSpawn::South => match self.direction {
                Direction::Up => BOTTOM_DESTINATION,
                Direction::Left => RIGHT_DESTINATION,
                Direction::Right => LEFT_DESTINATION,
            },
            VehicleSpawn::East => match self.direction {
                Direction::Up => RIGHT_DESTINATION,
                Direction::Left => UP_DESTINATION,
                Direction::Right => BOTTOM_DESTINATION,
            },
            VehicleSpawn::West => match self.direction {
                Direction::Up => LEFT_DESTINATION,
                Direction::Left => BOTTOM_DESTINATION,
                Direction::Right => UP_DESTINATION,
            },
        }
    }

    // Check if there's a vehicle ahead within safe distance
    pub fn is_vehicle_ahead(&self, vehicles: &VecDeque<Vehicle>) -> bool {
        for other in vehicles.iter() {
            if other.id == self.id {
                continue;
            }

            // Check if the other vehicle is in the same lane and ahead
            if self.is_vehicle_in_front(other) {
                let distance = self.calculate_distance_to(other);
                if distance < (SAFE_DISTANCE as f32 + VEHICLE_WIDTH as f32) {
                    return true;
                }
            }
        }
        false
    }

    // Check if another vehicle is in front of this one
    fn is_vehicle_in_front(&self, other: &Vehicle) -> bool {
        let tolerance = (VEHICLE_WIDTH as f32) / 2.0;

        match self.spawn {
            VehicleSpawn::North => {
                // Moving south initially, check if other vehicle is ahead (further south) and in same lane
                (other.position.0 - self.position.0).abs() < tolerance
                    && other.position.1 > self.position.1
            }
            VehicleSpawn::South => {
                // Moving north initially, check if other vehicle is ahead (further north) and in same lane
                (other.position.0 - self.position.0).abs() < tolerance
                    && other.position.1 < self.position.1
            }
            VehicleSpawn::East => {
                // Moving west initially, check if other vehicle is ahead (further west) and in same lane
                (other.position.1 - self.position.1).abs() < tolerance
                    && other.position.0 < self.position.0
            }
            VehicleSpawn::West => {
                // Moving east initially, check if other vehicle is ahead (further east) and in same lane
                (other.position.1 - self.position.1).abs() < tolerance
                    && other.position.0 > self.position.0
            }
        }
    }

    // Calculate distance to another vehicle
    fn calculate_distance_to(&self, other: &Vehicle) -> f32 {
        ((self.position.0 - other.position.0).powi(2)
            + (self.position.1 - other.position.1).powi(2))
        .sqrt()
    }

    // Updated move method that returns true if vehicle should be removed
    pub fn r#move(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        lights: &HashMap<String, TrafficLight>,
        vehicles: &VecDeque<Vehicle>, // Add vehicles parameter for collision detection
    ) -> bool {
        // Check if reached destination first
        if self.has_reached_destination() {
            return true;
        }

        let mut should_stop = false;

        // Check traffic light
        if let Some(light) = lights.get(self.spawn.as_str()) {
            if self.is_at_light() && !light.state {
                should_stop = true;
            }
        }

        // Check for vehicle ahead
        if !should_stop && self.is_vehicle_ahead(vehicles) {
            should_stop = true;
        }

        if self.speed > 0 && !should_stop {
            let offset: f32;
            match self.direction {
                Direction::Right => offset = CASE_SIZE as f32,
                Direction::Left => offset = 2.0 * CASE_SIZE as f32,
                Direction::Up => offset = 0.0,
            };
            if self.is_at_turn(offset) && !self.as_turned {
                self.as_turned = true;
                self.update_vector();
            }
            let movement_distance = self.speed as f32 * 1.0 / 60.0;
            self.position = (
                self.position.0 + self.vector.0 * movement_distance,
                self.position.1 + self.vector.1 * movement_distance,
            );
            self.update_shape();
        }

        self.render(canvas);
        false // Don't remove vehicle yet
    }

    fn get_light_position(&self) -> (f32, f32) {
        match self.spawn {
            VehicleSpawn::East => EAST_LIGHT,
            VehicleSpawn::South => SOUTH_LIGHT,
            VehicleSpawn::North => NORTH_LIGHT,
            VehicleSpawn::West => WEST_LIGHT,
        }
    }

    fn is_at_light(&self) -> bool {
        let x = (self.position.0 - self.get_light_position().0).abs();
        let y = (self.position.1 - self.get_light_position().1).abs();
        x != 0.0 && x < 2.5 || y != 0.0 && y < 2.5
    }

    fn is_at_turn(&self, offset: f32) -> bool {
        if offset == 0.0 {
            return true;
        };
        let x = self.position.0 - self.get_light_position().0;
        let y = self.position.1 - self.get_light_position().1;
        match self.spawn {
            VehicleSpawn::West => (x - offset).abs() <= 1.7,
            VehicleSpawn::North => (y - offset).abs() <= 1.7,
            VehicleSpawn::South => y + offset <= 1.7,
            VehicleSpawn::East => x + offset <= 1.7,
        }
    }

    fn update_vector(&mut self) {
        match self.direction {
            Direction::Left => {
                self.vector = (self.vector.1, -self.vector.0);
            }
            Direction::Right => {
                self.vector = (-self.vector.1, self.vector.0);
            }
            Direction::Up => {
                self.vector = self.vector;
            }
        }
    }

    fn update_shape(&mut self) {
        self.shape = Rect::new(
            self.position.0 as i32 - (VEHICLE_WIDTH / 2) as i32,
            self.position.1 as i32 - (VEHICLE_HEIGHT / 2) as i32,
            VEHICLE_WIDTH,
            VEHICLE_HEIGHT,
        )
    }

    pub fn brake(&mut self) {
        self.speed = 0;
    }

    pub fn accelerate(&mut self) {
        self.speed = VEHICLE_SPEED;
    }

    pub fn is_safe_distance(&self, other: &Vehicle) -> bool {
        let distance = ((self.position.0 - other.position.0).powi(2)
            + (self.position.1 - other.position.1).powi(2))
        .sqrt();
        distance > (self.speed as f32 * 1.0 / 60.0)
    }
}

// Function to check if a new vehicle can be spawned
pub fn can_spawn_vehicle(lane: &VecDeque<Vehicle>) -> bool {
    if lane.is_empty() {
        return true;
    }

    // Get the last spawned vehicle (most recent)
    if let Some(last_vehicle) = lane.back() {
        // Calculate distance from spawn point to last vehicle
        let spawn_distance = match last_vehicle.spawn {
            VehicleSpawn::North => {
                // For north spawn, check if last vehicle has moved far enough down
                let spawn_pos = crate::config::UP_SPAWN;
                (last_vehicle.position.1 - spawn_pos.1).abs()
            }
            VehicleSpawn::South => {
                // For south spawn, check if last vehicle has moved far enough up
                let spawn_pos = crate::config::BOTTOM_SPAWN;
                (spawn_pos.1 - last_vehicle.position.1).abs()
            }
            VehicleSpawn::East => {
                // For east spawn, check if last vehicle has moved far enough left
                let spawn_pos = crate::config::RIGHT_SPAWN;
                (spawn_pos.0 - last_vehicle.position.0).abs()
            }
            VehicleSpawn::West => {
                // For west spawn, check if last vehicle has moved far enough right
                let spawn_pos = crate::config::LEFT_SPAWN;
                (last_vehicle.position.0 - spawn_pos.0).abs()
            }
        };

        // Ensure there's enough distance (safe distance + vehicle width)
        let min_distance = (SAFE_DISTANCE + VEHICLE_WIDTH) as f32;
        spawn_distance >= min_distance
    } else {
        true
    }
}
