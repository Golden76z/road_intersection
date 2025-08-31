use rand::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, render::Canvas};

use crate::{
    config::{
        BOTTOM_DESTINATION, BOTTOM_SPAWN, LEFT_DESTINATION, LEFT_SPAWN, RIGHT_DESTINATION,
        RIGHT_SPAWN, TrafficLanes, UP_DESTINATION, UP_SPAWN,
    },
    simulation::{Vehicle,VehicleSpawn, spawn_vehicle},
};
use crate::config::{Direction, BOTTOM_VECTOR, LEFT_VECTOR, RIGHT_VECTOR, TOP_VECTOR};
use crate::render::Renderer;

pub fn input_listener(event: Event, renderer: &mut Renderer) -> Result<(), String> {
    // Input listening
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => return Err("Program end".to_string()),
        Event::KeyDown {
            keycode: Some(Keycode::N),
            ..
        } => { 
            renderer.change_state("North");
        Ok(())},

        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => { renderer.change_state("West");
        Ok(())},

        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => { renderer.change_state("South"); 
        Ok(())},

        Event::KeyDown {
            keycode: Some(Keycode::E),
            ..
        } => { renderer.change_state("East");
        Ok(())},

        // Listening for the DOWN keypress
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            println!("Down arrow pressed");

            // Lock the bottom lane for checking and potentially adding
            let mut bottom_lane = renderer.lanes.bottom.lock().unwrap();
            if spawn_vehicle(&*bottom_lane) {
                bottom_lane.push_back(Vehicle::new(
                    1,
                    BOTTOM_SPAWN,
                    TOP_VECTOR,
                    Direction::random(),
                    VehicleSpawn::South,
                ));
            }

            Ok(())
        }

        // Listening for the UP keypress
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            println!("Up arrow pressed");

            // Lock the up lane for checking and potentially adding
            let mut up_lane = renderer.lanes.up.lock().unwrap();
            if spawn_vehicle(&*up_lane) {
                up_lane.push_back(Vehicle::new(
                    1,
                    UP_SPAWN,
                    BOTTOM_VECTOR,
                    Direction::random(),
                    VehicleSpawn::North,
                ));
            }

            Ok(())
        }

        // Listening for the LEFT keypress
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            println!("Left arrow pressed");

            // Lock the left lane for checking and potentially adding
            let mut left_lane = renderer.lanes.left.lock().unwrap();
            if spawn_vehicle(&*left_lane) {
                left_lane.push_back(Vehicle::new(
                    1,
                    LEFT_SPAWN,
                    RIGHT_VECTOR,
                    Direction::random(),
                    VehicleSpawn::West,
                ));
            }

            Ok(())
        }

        // Listening for the RIGHT keypress
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            println!("Right arrow pressed");

            // Lock the right lane for checking and potentially adding
            let mut right_lane = renderer.lanes.right.lock().unwrap();
            if spawn_vehicle(&*right_lane) {
                right_lane.push_back(Vehicle::new(
                    1,
                    RIGHT_SPAWN,
                    LEFT_VECTOR,
                    Direction::Right,
                    VehicleSpawn::East
                ));
            }

            Ok(())
        }

        // Listening to the R keypress - Random dirrection
        Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
        } => {
            // Generating a random number to randominze the direction
            let mut rng = rand::rng();
            let rand_num = rng.random_range(0..4);

            match rand_num {
                0 => {
                    println!("Random generated Left Vehicle");
                }
                1 => {
                    println!("Random generated Right Vehicle");
                }
                2 => {
                    println!("Random generated Up Vehicle");
                }
                _ => {
                    println!("Random generated Down Vehicle");
                }
            }

            Ok(())
        }

        _ => Ok(()),
    }
}
