use std::time::Duration;

mod config;
mod input;
mod render;
mod simulation;

use crate::{
    config::{CANVA_HEIGHT, CANVA_WIDTH},
    render::Renderer,
};
use input::input_listener;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road_intersection", CANVA_WIDTH, CANVA_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    // Creating the new renderer - To print on the screen
    let mut renderer = Renderer::new(window).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        renderer.canvas.clear();

        // Input listener - Vehicle spawning
        for event in event_pump.poll_iter() {
            match input_listener(event, &renderer.lanes) {
                Ok(()) => {}
                Err(msg) => {
                    println!("{}", msg);
                    break 'running;
                }
            }
        }

        // Drawing the road and traffic lights
        renderer.draw().unwrap();

        // Drawing the vehicles
        renderer.draw_vehicles();

        renderer.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
