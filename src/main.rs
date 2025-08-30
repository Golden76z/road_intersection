use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect};
use std::time::Duration;

mod config;
mod input;
mod render;
mod simulation;

use input::input_listener;
use render::sdl_renderer;
use simulation::{controller, road, traffic_light, vehicle};

use crate::config::{TrafficLanes, VEHICLE_HEIGHT, VEHICLE_WIDTH};

pub fn main() {
    let lanes = TrafficLanes::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road_intersection", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        // Input listener - Vehicle spawning
        for event in event_pump.poll_iter() {
            match input_listener(event, &lanes, &mut canvas) {
                Ok(()) => {}
                Err(msg) => {
                    println!("{}", msg);
                    break 'running;
                }
            }
        }

        for item in lanes.bottom.lock().unwrap().iter_mut() {
            println!("Item: {:?}", item);
            item.update_position();
            item.render(&mut canvas, VEHICLE_WIDTH, VEHICLE_HEIGHT);
        }
        // Rest of the game loop
        canvas.set_draw_color(Color::RGB(0, 0, 0));

        // canvas.draw_rect(rect::Rect::new(500, 500, 50, 50));
        // canvas.draw_line(rect::Point::new(500, 0), rect::Point::new(500, 1000));

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
