use std::collections::HashMap;
use sdl2;
use sdl2::libc::abs;
use sdl2::rect::Rect;
use crate::config::{Direction, CASE_SIZE, EAST_LIGHT, NORTH_LIGHT, SOUTH_LIGHT, VEHICLE_HEIGHT, VEHICLE_SPEED, VEHICLE_WIDTH, WEST_LIGHT};
use crate::simulation::TrafficLight;

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
            VehicleSpawn::West => "West"
        }
    }
}

#[derive(Debug)]
pub struct Vehicle {
    pub id: i32,
    pub position: (f32, f32),
    pub vector: (f32,f32),
    pub speed: u16,
    pub spawn: VehicleSpawn,
    pub direction: Direction,
    pub shape: sdl2::rect::Rect,
    pub as_turned: bool
}

impl<'a> Vehicle {
    pub fn new(
        id: i32,
        position: (f32, f32),
        vector:(f32,f32),
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
            as_turned: false
        };
        vehicle.accelerate();
        vehicle
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.direction.color());
        let _ = canvas.fill_rect(self.shape);
    }

    // delta_time = 60 fps
    pub fn r#move(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, lights: &HashMap<String,TrafficLight>) {
        if self.speed > 0 && let Some(light) = lights.get(self.spawn.as_str()) {
            if self.is_at_light() && !light.state {
                self.position = self.position;
            } else {
                let offset :f32;
                match self.direction {
                    Direction::Right => offset = CASE_SIZE as f32,
                    Direction::Left => offset = 2.0 * CASE_SIZE as f32,
                    Direction::Up => offset = 0.0
                };
                if self.is_at_turn(offset) && !self.as_turned {
                    self.as_turned = true;
                    self.update_vector();
                }
                let movement_distance = self.speed as f32 * 1.0 / 60.0;
                self.position = (self.position.0 + self.vector.0 * movement_distance,self.position.1 + self.vector.1 * movement_distance) ;
                self.update_shape();
            }
        }

        self.render(canvas);
    }

    fn get_light_position(&self) -> (f32,f32) {
        match self.spawn {
            VehicleSpawn::East => EAST_LIGHT,
            VehicleSpawn::South => SOUTH_LIGHT,
            VehicleSpawn::North => NORTH_LIGHT,
            VehicleSpawn::West => WEST_LIGHT
        }
    }

    fn is_at_light(&self) -> bool {
        let x = (self.position.0 - self.get_light_position().0).abs();
        let y = (self.position.1 - self.get_light_position().1).abs();
        x != 0.0 && x < 2.5 ||  y != 0.0 && y < 2.5
    }

    fn is_at_turn(&self,offset: f32) -> bool {
        if offset == 0.0 {return true};
        let x = self.position.0 - self.get_light_position().0 ;
        let y = self.position.1 - self.get_light_position().1;
        match self.spawn {
            VehicleSpawn::West => {
                (x - offset).abs() <= 1.7
            },
            VehicleSpawn::North => {
                (y - offset).abs() <= 1.7
            },
            VehicleSpawn::South => {
                y + offset <= 1.7
            } ,
            VehicleSpawn::East => {
                x + offset <= 1.7
            }
        }
    }

    fn update_vector(&mut self) {
        match self.direction {
            Direction::Left => {
                self.vector = (self.vector.1,-self.vector.0);
            },
            Direction::Right => {
                self.vector = (-self.vector.1,self.vector.0);
            },
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

    // pub fn find_closest_vehicle_ahead(&'a self, vehicles: &'a [Vehicle]) -> Option<&'a Vehicle> {
    //     vehicles
    //         .iter()
    //         .filter(|&other| other.id != self.id)
    //         .filter(|&other| match self.direction {
    //             VehicleDirection::North => {
    //                 other.position.1 < self.position.1
    //                     && (other.position.0 - self.position.0).abs() < VEHICLE_WIDTH as f32
    //             }
    //             VehicleDirection::South => {
    //                 other.position.1 > self.position.1
    //                     && (other.position.0 - self.position.0).abs() < VEHICLE_WIDTH as f32
    //             }
    //             VehicleDirection::East => {
    //                 other.position.0 > self.position.0
    //                     && (other.position.1 - self.position.1).abs() < VEHICLE_WIDTH as f32
    //             }
    //             VehicleDirection::West => {
    //                 other.position.0 < self.position.0
    //                     && (other.position.1 - self.position.1).abs() < VEHICLE_WIDTH as f32
    //             }
    //         })
    //         .min_by(|&a, &b| {
    //             let dist_a = ((self.position.0 - a.position.0).powi(2)
    //                 + (self.position.1 - a.position.1).powi(2))
    //             .sqrt();
    //             let dist_b = ((self.position.0 - b.position.0).powi(2)
    //                 + (self.position.1 - b.position.1).powi(2))
    //             .sqrt();
    //             dist_a.partial_cmp(&dist_b).unwrap()
    //         })
    // }
}

