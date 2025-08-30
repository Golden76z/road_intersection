// extern crate sdl2;

use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect};
use std::time::Duration;
use sdl2::rect::Rect;

mod config;
mod input;
mod render;
mod simulation;
mod config;

use input::input_listener;
use render::sdl_renderer;
use simulation::{controller, road, traffic_light, vehicle};
use crate::config::*;
use crate::render::Renderer;

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

    let mut renderer = Renderer::new(window).unwrap();
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
        // The rest of the game loop goes here...
        renderer.draw().unwrap();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
