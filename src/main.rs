// extern crate sdl2;

use std::collections::HashMap;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::rect::Rect;

mod input;
mod render;
mod simulation;
mod config;

use input::keyboard;
use render::sdl_renderer;
use simulation::{controller, road, traffic_light, vehicle};
use crate::config::*;
use crate::render::Renderer;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("road_intersection", GRID_WIDTH * CASE_SIZE, GRID_HEIGHT * CASE_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = Renderer::new(window).unwrap();
    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;

    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => renderer.change_state("North"),

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => renderer.change_state("West"),

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => renderer.change_state("South"),

                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => renderer.change_state("East"),
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        renderer.draw().unwrap();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
