use crate::config::*;
use crate::simulation::{TrafficLanes, TrafficLight};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct Renderer {
    pub canvas: WindowCanvas,
    pub lights: HashMap<String, TrafficLight>,
    pub lanes: TrafficLanes,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let lights: HashMap<String, TrafficLight> = HashMap::from([
            (
                String::from("North"),
                TrafficLight::new(Rect::new(
                    (GRID_WIDTH_PIXELS / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    (GRID_HEIGHT_PIXELS / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
            (
                String::from("East"),
                TrafficLight::new(Rect::new(
                    (GRID_WIDTH_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
                    (GRID_HEIGHT_PIXELS / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
            (
                String::from("West"),
                TrafficLight::new(Rect::new(
                    (GRID_WIDTH_PIXELS / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    (GRID_HEIGHT_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
            (
                String::from("South"),
                TrafficLight::new(Rect::new(
                    (GRID_WIDTH_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
                    (GRID_HEIGHT_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
        ]);

        let lanes = TrafficLanes::new();

        Ok(Renderer {
            canvas,
            lights,
            lanes,
        })
    }
    //Lights
    pub fn change_state(&mut self, s: &str) {
        if let Some(light) = self.lights.get_mut(s) {
            light.change_state();
        }
    }

    //Only for straight line
    fn draw_dotted_line(&mut self, start: (i32, i32), end: (i32, i32)) -> Result<(), String> {
        self.canvas.set_draw_color(Color::WHITE);
        match start.0 == end.0 {
            true => {
                let x = start.0;
                let mut min = min(start.1, end.1);
                let max = max(start.1, end.1);
                let mut i = min + 10;
                while i < max {
                    self.canvas
                        .draw_line(Point::new(x, min), Point::new(x, i))?;
                    (min, i) = (i + 10, i + 20)
                }
                if min < max {
                    self.canvas
                        .draw_line(Point::new(x, min), Point::new(x, max))?;
                }
                Ok(())
            }
            false => {
                let y = start.1;
                let mut min = min(start.0, end.0);
                let max = max(start.0, end.0);
                let mut i = min + 10;
                while i < max {
                    self.canvas
                        .draw_line(Point::new(min, y), Point::new(i, y))?;
                    (min, i) = (i + 10, i + 20)
                }
                if min < max {
                    self.canvas
                        .draw_line(Point::new(min, y), Point::new(max, y))?;
                }
                Ok(())
            }
        }
    }
    fn init_map(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::GREY);

        self.canvas.draw_rect(Rect::new(
            -1,
            -1,
            GRID_WIDTH_PIXELS / 2 - CASE_SIZE,
            GRID_HEIGHT_PIXELS / 2 - CASE_SIZE,
        ))?;
        self.canvas.draw_rect(Rect::new(
            (GRID_WIDTH_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
            -1,
            GRID_WIDTH_PIXELS / 2 - CASE_SIZE,
            GRID_HEIGHT_PIXELS / 2 - CASE_SIZE,
        ))?;
        self.canvas.draw_rect(Rect::new(
            -1,
            (GRID_HEIGHT_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
            GRID_WIDTH_PIXELS / 2 - CASE_SIZE,
            GRID_HEIGHT_PIXELS / 2 - CASE_SIZE,
        ))?;
        self.canvas.draw_rect(Rect::new(
            (GRID_WIDTH_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
            (GRID_HEIGHT_PIXELS / 2 + CASE_SIZE + 1).cast_signed(),
            GRID_WIDTH_PIXELS / 2 - CASE_SIZE,
            GRID_HEIGHT_PIXELS / 2 - CASE_SIZE,
        ))?;

        self.draw_dotted_line(
            (-5, (GRID_HEIGHT_PIXELS / 2).cast_signed()),
            (
                (GRID_WIDTH_PIXELS / 2 - CASE_SIZE).cast_signed(),
                (GRID_HEIGHT_PIXELS / 2).cast_signed(),
            ),
        )?;
        self.draw_dotted_line(
            ((GRID_WIDTH_PIXELS / 2).cast_signed(), -5),
            (
                (GRID_WIDTH_PIXELS / 2).cast_signed(),
                (GRID_HEIGHT_PIXELS / 2 - CASE_SIZE).cast_signed(),
            ),
        )?;
        self.draw_dotted_line(
            (
                GRID_WIDTH_PIXELS.cast_signed(),
                (GRID_HEIGHT_PIXELS / 2).cast_signed(),
            ),
            (
                (GRID_WIDTH_PIXELS / 2 + CASE_SIZE).cast_signed(),
                (GRID_HEIGHT_PIXELS / 2).cast_signed(),
            ),
        )?;
        self.draw_dotted_line(
            (
                (GRID_WIDTH_PIXELS / 2).cast_signed(),
                GRID_HEIGHT_PIXELS.cast_signed(),
            ),
            (
                (GRID_WIDTH_PIXELS / 2).cast_signed(),
                (GRID_HEIGHT_PIXELS / 2 + CASE_SIZE).cast_signed(),
            ),
        )?;

        Ok(())
    }

    pub fn draw_vehicles(&mut self) {
        // Drawing the up lane vehicles
        for item in self.lanes.up.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas);
        }

        // Drawing the bottom lane vehicles
        for item in self.lanes.bottom.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas);
        }

        // Drawing the left lane vehicles
        for item in self.lanes.left.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas);
        }

        // Drawing the right lane vehicles
        for item in self.lanes.right.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas);
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.init_map()?;
        for light in self.lights.values_mut() {
            light.draw(&mut self.canvas)?;
        }

        Ok(())
    }
}
