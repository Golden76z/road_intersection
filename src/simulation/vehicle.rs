use sdl2;

use crate::config::{VEHICLE_HEIGHT, VEHICLE_SPEED, VEHICLE_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleDirection {
    North,
    South,
    East,
    West,
}

pub struct Vehicle {
    pub id: i32,
    pub position: (f32, f32),
    pub target: (f32, f32),
    pub speed: u16,
    pub direction: VehicleDirection,
    pub shape: sdl2::rect::Rect,
}

impl<'a> Vehicle {
    pub fn new(
        id: i32,
        position: (f32, f32),
        target: (f32, f32),
        direction: VehicleDirection,
    ) -> Self {
        Vehicle {
            id,
            position,
            target,
            speed: VEHICLE_SPEED,
            direction,
            shape: match direction {
                VehicleDirection::North | VehicleDirection::South => sdl2::rect::Rect::new(
                    position.0 as i32 - (VEHICLE_WIDTH / 2) as i32,
                    position.1 as i32 - (VEHICLE_HEIGHT / 2) as i32,
                    VEHICLE_WIDTH,
                    VEHICLE_HEIGHT,
                ),
                VehicleDirection::East | VehicleDirection::West => sdl2::rect::Rect::new(
                    position.0 as i32 - (VEHICLE_HEIGHT / 2) as i32,
                    position.1 as i32 - (VEHICLE_WIDTH / 2) as i32,
                    VEHICLE_HEIGHT,
                    VEHICLE_WIDTH,
                ),
            },
        }
    }

    pub fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        let _ = canvas.fill_rect(self.shape);
    }

    // delta_time = 60 fps
    pub fn r#move(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.speed > 0 {
            let direction: (f32, f32) = (
                self.target.0 - self.position.0,
                self.target.1 - self.position.1,
            );
            let distance = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

            if distance > 0.0 {
                let normalized_x = direction.0 / distance;
                let normalized_y = direction.1 / distance;

                let movement_distance = self.speed as f32 * 1.0 / 60.0;
                
                // Vérifier si on va dépasser la cible
                if distance <= movement_distance {
                    // On arrive à la cible, s'arrêter là
                    self.position = self.target;
                } else {
                    // Mouvement normal
                    self.position.0 += normalized_x * movement_distance;
                    self.position.1 += normalized_y * movement_distance;
                }
                
                // IMPORTANT: Mettre à jour le shape après avoir changé la position
                self.update_shape();
            }
        }

        self.render(canvas);
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

    fn update_shape(&mut self) {
        self.shape = match self.direction {
            VehicleDirection::North | VehicleDirection::South => sdl2::rect::Rect::new(
                self.position.0 as i32 - (VEHICLE_WIDTH / 2) as i32,
                self.position.1 as i32 - (VEHICLE_HEIGHT / 2) as i32,
                VEHICLE_WIDTH,
                VEHICLE_HEIGHT,
            ),
            VehicleDirection::East | VehicleDirection::West => sdl2::rect::Rect::new(
                self.position.0 as i32 - (VEHICLE_HEIGHT / 2) as i32,
                self.position.1 as i32 - (VEHICLE_WIDTH / 2) as i32,
                VEHICLE_HEIGHT,
                VEHICLE_WIDTH,
            ),
        };
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
    ) -> Option<&'a Vehicle> {
        vehicles
            .iter()
            .filter(|&other| other.id != self.id)
            .filter(|&other| match self.direction {
                VehicleDirection::North => {
                    other.position.1 < self.position.1
                        && (other.position.0 - self.position.0).abs() < VEHICLE_WIDTH as f32
                }
                VehicleDirection::South => {
                    other.position.1 > self.position.1
                        && (other.position.0 - self.position.0).abs() < VEHICLE_WIDTH as f32
                }
                VehicleDirection::East => {
                    other.position.0 > self.position.0
                        && (other.position.1 - self.position.1).abs() < VEHICLE_WIDTH as f32
                }
                VehicleDirection::West => {
                    other.position.0 < self.position.0
                        && (other.position.1 - self.position.1).abs() < VEHICLE_WIDTH as f32
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
}

