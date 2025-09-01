use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::render::Renderer;

// Function that will listen to user's inputs
pub fn input_listener(event: Event, renderer: &mut Renderer) -> Result<(), String> {
    // Input listening
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => return Err("<---------- Program end ---------->".to_string()),

        // Listening for the DOWN keypress
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            // println!("Down arrow pressed");
            renderer.lanes.spawn_vehicle("down");

            Ok(())
        }

        // Listening for the UP keypress
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            // println!("Up arrow pressed");
            renderer.lanes.spawn_vehicle("up");

            Ok(())
        }

        // Listening for the LEFT keypress
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            // println!("Left arrow pressed");
            renderer.lanes.spawn_vehicle("left");

            Ok(())
        }

        // Listening for the RIGHT keypress
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            // println!("Right arrow pressed");
            renderer.lanes.spawn_vehicle("right");

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
                    println!("-> Random generated Left Vehicle");
                    renderer.lanes.spawn_vehicle("left");
                }
                1 => {
                    println!("-> Random generated Right Vehicle");
                    renderer.lanes.spawn_vehicle("right");
                }
                2 => {
                    println!("-> Random generated Up Vehicle");
                    renderer.lanes.spawn_vehicle("up");
                }
                _ => {
                    println!("-> Random generated Down Vehicle");
                    renderer.lanes.spawn_vehicle("down");
                }
            }

            Ok(())
        }

        // DEBUG INPUT - TRIGGERING TRAFFIC LIGHTS MANUALLY

        // Event::KeyDown {
        //     keycode: Some(Keycode::N),
        //     ..
        // } => {
        //     renderer.change_state("North");
        //     Ok(())
        // }
        //
        // Event::KeyDown {
        //     keycode: Some(Keycode::W),
        //     ..
        // } => {
        //     renderer.change_state("West");
        //     Ok(())
        // }
        //
        // Event::KeyDown {
        //     keycode: Some(Keycode::S),
        //     ..
        // } => {
        //     renderer.change_state("South");
        //     Ok(())
        // }
        //
        // Event::KeyDown {
        //     keycode: Some(Keycode::E),
        //     ..
        // } => {
        //     renderer.change_state("East");
        //     Ok(())
        // }

        // In case of no input from user, no error generated
        _ => Ok(()),
    }
}
