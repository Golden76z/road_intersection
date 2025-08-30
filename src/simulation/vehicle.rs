use sdl2;

use crate::config::VEHICLE_SPEED;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VehicleDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
pub struct Vehicle {
    pub id: i32,
    pub position: (f32, f32),
    pub target: (f32, f32),
    pub speed: u16,
    pub direction: VehicleDirection,
}

impl<'a> Vehicle {
    pub fn new(
        id: i32,
        position: (f32, f32),
        target: (f32, f32),
        speed: u16,
        direction: VehicleDirection,
    ) -> Self {
        let mut new_vehicle = Vehicle {
            id,
            position,
            target,
            speed,
            direction,
        };
        new_vehicle.accelerate();
        new_vehicle
    }

    // delta_time = 60 fps
    pub fn update_position(&mut self) {
        let direction: (f32, f32) = (
            self.target.0 - self.position.0,
            self.target.1 - self.position.1,
        );
        let distance = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

        if distance > 0.0 {
            let normalized_x = direction.0 / distance;
            let normalized_y = direction.1 / distance;

            self.position.0 += normalized_x * self.speed as f32 * 1.0 / 60.0;
            self.position.1 += normalized_y * self.speed as f32 * 1.0 / 60.0;
        }
    }

    pub fn update_target(&mut self, target: (f32, f32)) {
        self.target = target;
        self.update_direction();
    }

    pub fn update_direction(&mut self) {
        if self.target.0 > self.position.0 {
            self.direction = VehicleDirection::East;
        } else if self.target.0 < self.position.0 {
            self.direction = VehicleDirection::West;
        } else if self.target.1 > self.position.1 {
            self.direction = VehicleDirection::South;
        } else if self.target.1 < self.position.1 {
            self.direction = VehicleDirection::North;
        }
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

    pub fn find_closest_vehicle_ahead(
        &'a self,
        vehicles: &'a [Vehicle],
        vehicle_width: u32,
    ) -> Option<&'a Vehicle> {
        vehicles
            .iter()
            .filter(|&other| other.id != self.id)
            .filter(|&other| match self.direction {
                VehicleDirection::North => {
                    other.position.1 < self.position.1
                        && (other.position.0 - self.position.0).abs() < vehicle_width as f32
                }
                VehicleDirection::South => {
                    other.position.1 > self.position.1
                        && (other.position.0 - self.position.0).abs() < vehicle_width as f32
                }
                VehicleDirection::East => {
                    other.position.0 > self.position.0
                        && (other.position.1 - self.position.1).abs() < vehicle_width as f32
                }
                VehicleDirection::West => {
                    other.position.0 < self.position.0
                        && (other.position.1 - self.position.1).abs() < vehicle_width as f32
                }
            })
            .min_by(|&a, &b| {
                let dist_a = ((self.position.0 - a.position.0).powi(2)
                    + (self.position.1 - a.position.1).powi(2))
                .sqrt();
                let dist_b = ((self.position.0 - b.position.0).powi(2)
                    + (self.position.1 - b.position.1).powi(2))
                .sqrt();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
    }

    pub fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        vehicle_width: u32,
        vehicle_height: u32,
    ) {
        let rect = match self.direction {
            VehicleDirection::North | VehicleDirection::South => sdl2::rect::Rect::new(
                self.position.0 as i32 - (vehicle_width / 2) as i32,
                self.position.1 as i32 - (vehicle_height / 2) as i32,
                vehicle_width,
                vehicle_height,
            ),
            VehicleDirection::East | VehicleDirection::West => sdl2::rect::Rect::new(
                self.position.0 as i32 - (vehicle_height / 2) as i32,
                self.position.1 as i32 - (vehicle_width / 2) as i32,
                vehicle_height,
                vehicle_width,
            ),
        };
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        let _ = canvas.fill_rect(rect);
    }
}

