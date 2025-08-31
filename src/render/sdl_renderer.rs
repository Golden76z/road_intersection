use crate::config::*;
use crate::simulation::{TrafficLight};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

pub struct Renderer {
    pub canvas: WindowCanvas,
    pub lights: HashMap<String, TrafficLight>,
    pub lanes: TrafficLanes,
    pub waiting: HashMap<String, HashSet<i32>>
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let lights: HashMap<String, TrafficLight> = HashMap::from([
            (
                String::from("North"),
                TrafficLight::new(Rect::new(
                    (CANVA_WIDTH / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    (CANVA_HEIGHT / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
            (
                String::from("East"),
                TrafficLight::new(Rect::new(
                    (CANVA_WIDTH / 2 + CASE_SIZE + 1).cast_signed(),
                    (CANVA_HEIGHT / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
            (
                String::from("West"),
                TrafficLight::new(Rect::new(
                    (CANVA_WIDTH / 2 - 2 * CASE_SIZE - 1).cast_signed(),
                    (CANVA_HEIGHT / 2 + CASE_SIZE + 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
            (
                String::from("South"),
                TrafficLight::new(Rect::new(
                    (CANVA_WIDTH / 2 + CASE_SIZE + 1).cast_signed(),
                    (CANVA_HEIGHT / 2 + CASE_SIZE + 1).cast_signed(),
                    CASE_SIZE,
                    CASE_SIZE,
                )),
            ),
        ]);

        let lanes = TrafficLanes::new();
        let waiting : HashMap<String,HashSet<i32>> = HashMap::from([(String::from("South"),HashSet::new()),(String::from("North"),HashSet::new()),(String::from("East"),HashSet::new()),(String::from("West"),HashSet::new())]);
        Ok(Renderer {
            canvas,
            lights,
            lanes,
            waiting
        })
    }
    //Lights
    pub fn change_state(&mut self, s: &str,state: Option<bool>) {
        if let Some(light) = self.lights.get_mut(s) {
            light.change_state(state)
        }
    }

    fn update_lights(&mut self) {
        let mut max : (&String,usize) = (&String::new(),0);
        for (lane,vehicles) in self.waiting.iter_mut() {
            if vehicles.len() == 0 {
                if let Some(light) = self.lights.get_mut(lane) {
                    light.change_state(Some(false))
                }
            } else {
                if max.1 < vehicles.len() {
                    if let Some(light) = self.lights.get_mut(max.0) {
                        light.change_state(Some(false))
                    }
                    max = (lane, vehicles.len())
                }
            }
        }
        if let Some(light) = self.lights.get_mut(max.0) {
            light.change_state(Some(true))
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
            CANVA_WIDTH / 2 - CASE_SIZE,
            CANVA_HEIGHT / 2 - CASE_SIZE,
        ))?;
        self.canvas.draw_rect(Rect::new(
            (CANVA_WIDTH / 2 + CASE_SIZE + 1).cast_signed(),
            -1,
            CANVA_WIDTH / 2 - CASE_SIZE,
            CANVA_HEIGHT / 2 - CASE_SIZE,
        ))?;
        self.canvas.draw_rect(Rect::new(
            -1,
            (CANVA_HEIGHT / 2 + CASE_SIZE + 1).cast_signed(),
            CANVA_WIDTH / 2 - CASE_SIZE,
            CANVA_HEIGHT / 2 - CASE_SIZE,
        ))?;
        self.canvas.draw_rect(Rect::new(
            (CANVA_WIDTH / 2 + CASE_SIZE + 1).cast_signed(),
            (CANVA_HEIGHT / 2 + CASE_SIZE + 1).cast_signed(),
            CANVA_WIDTH / 2 - CASE_SIZE,
            CANVA_HEIGHT / 2 - CASE_SIZE,
        ))?;

        self.draw_dotted_line(
            (-5, (CANVA_HEIGHT / 2).cast_signed()),
            (
                (CANVA_WIDTH / 2 - CASE_SIZE).cast_signed(),
                (CANVA_HEIGHT / 2).cast_signed(),
            ),
        )?;
        self.draw_dotted_line(
            ((CANVA_WIDTH / 2).cast_signed(), -5),
            (
                (CANVA_WIDTH / 2).cast_signed(),
                (CANVA_HEIGHT / 2 - CASE_SIZE).cast_signed(),
            ),
        )?;
        self.draw_dotted_line(
            (
                CANVA_WIDTH.cast_signed(),
                (CANVA_HEIGHT / 2).cast_signed(),
            ),
            (
                (CANVA_WIDTH / 2 + CASE_SIZE).cast_signed(),
                (CANVA_HEIGHT / 2).cast_signed(),
            ),
        )?;
        self.draw_dotted_line(
            (
                (CANVA_WIDTH / 2).cast_signed(),
                CANVA_HEIGHT.cast_signed(),
            ),
            (
                (CANVA_WIDTH / 2).cast_signed(),
                (CANVA_HEIGHT / 2 + CASE_SIZE).cast_signed(),
            ),
        )?;

        Ok(())
    }

    pub fn draw_vehicles(&mut self) {
        // Drawing the up lane vehicles
        for item in self.lanes.up.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas, &self.lights, &mut self.waiting);
        }

        // Drawing the bottom lane vehicles
        for item in self.lanes.bottom.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas, &self.lights, &mut self.waiting);
        }

        // Drawing the left lane vehicles
        for item in self.lanes.left.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas, &self.lights, &mut self.waiting);
        }

        // Drawing the right lane vehicles
        for item in self.lanes.right.lock().unwrap().iter_mut() {
            item.r#move(&mut self.canvas, &self.lights, &mut self.waiting);
        }
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.init_map()?;
        self.update_lights();
        for light in self.lights.values_mut() {
            light.draw(&mut self.canvas)?;
        }

        Ok(())
    }
}
