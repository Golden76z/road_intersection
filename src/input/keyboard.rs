use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn input_listener(event: Event) -> Result<(), String> {
    // Input listening
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => return Err("Program end".to_string()),
        // Listening for the UP keypress
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            println!("Up arrow pressed");
            Ok(())
        }

        // Listening for the DOWN keypress
        Event::KeyDown {
            keycode: Some(Keycode::Down),
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

        _ => Ok(()),
    }
}
