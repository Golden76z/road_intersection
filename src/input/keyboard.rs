use rand::prelude::*;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, render::Canvas};

use crate::{
    config::{
        BOTTOM_DESTINATION, BOTTOM_SPAWN, LEFT_DESTINATION, LEFT_SPAWN, RIGHT_DESTINATION,
        RIGHT_SPAWN, TrafficLanes, UP_DESTINATION, UP_SPAWN,
    },
    simulation::{Vehicle, VehicleDirection, spawn_vehicle},
};

pub fn input_listener(event: Event, lanes: &TrafficLanes) -> Result<(), String> {
    // Input listening
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => return Err("Program end".to_string()),

        // Listening for the DOWN keypress
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            println!("Down arrow pressed");

            // Lock the bottom lane for checking and potentially adding
            let mut bottom_lane = lanes.bottom.lock().unwrap();
            if spawn_vehicle(&*bottom_lane) {
                bottom_lane.push_back(Vehicle::new(
                    1,
                    BOTTOM_SPAWN,
                    BOTTOM_DESTINATION,
                    VehicleDirection::North,
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
            let mut up_lane = lanes.up.lock().unwrap();
            if spawn_vehicle(&*up_lane) {
                up_lane.push_back(Vehicle::new(
                    1,
                    UP_SPAWN,
                    UP_DESTINATION,
                    VehicleDirection::South,
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
            let mut left_lane = lanes.left.lock().unwrap();
            if spawn_vehicle(&*left_lane) {
                left_lane.push_back(Vehicle::new(
                    1,
                    LEFT_SPAWN,
                    LEFT_DESTINATION,
                    VehicleDirection::East,
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
            let mut right_lane = lanes.right.lock().unwrap();
            if spawn_vehicle(&*right_lane) {
                right_lane.push_back(Vehicle::new(
                    1,
                    RIGHT_SPAWN,
                    RIGHT_DESTINATION,
                    VehicleDirection::West,
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
