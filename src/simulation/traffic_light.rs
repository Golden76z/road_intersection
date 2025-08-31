use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TrafficLight {
    pub(crate) rect: Rect,
    pub(crate) state: bool,
}

impl TrafficLight {
    pub fn new(rect: Rect) -> Self {
        Self { rect, state: false }
    }

    pub fn change_state(&mut self) {
        self.state = !self.state;
    }
    pub fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        match self.state {
            true => canvas.set_draw_color(Color::GREEN),
            false => {
                canvas.set_draw_color(Color::RED);
            }
        };
        canvas.draw_rect(self.rect)?;

        Ok(())
    }
}
