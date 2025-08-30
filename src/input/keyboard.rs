use std::thread::spawn;

use rand::prelude::*;

use sdl2::keyboard::Keycode;
use sdl2::{event::Event, render::Canvas};

// use crate::config::{
//     BOTTOM, BOTTOM_DESTINATION, BOTTOM_SPAWN, CANVA_WIDTH, VEHICLE_SPEED, VEHICLE_WIDTH,
// };
use crate::{
    config::{
        BOTTOM_DESTINATION, BOTTOM_SPAWN, TrafficLanes, VEHICLE_HEIGHT, VEHICLE_SPEED,
        VEHICLE_WIDTH,
    },
    simulation::{Vehicle, VehicleDirection, spawn_vehicle},
};

pub fn input_listener(
    event: Event,
    lanes: &TrafficLanes,
    canvas: &mut Canvas<sdl2::video::Window>,
) -> Result<(), String> {
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
            println!("Up arrow pressed");

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
            let length = bottom_lane.len() - 1;
            bottom_lane[length].accelerate();
            Ok(())
        }

        // Listening for the UP keypress
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            println!("Down arrow pressed");
            Ok(())
        }

        // Listening for the LEFT keypress
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            println!("Left arrow pressed");
            Ok(())
        }

        // Listening for the RIGHT keypress
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            println!("Right arrow pressed");
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
