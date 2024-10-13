use rsautogui::mouse::{self, Button, Speed};

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16
}

pub struct Operator {
    pub area: Rect,
    pub scale: f64
}

impl Operator {
    pub fn new(x: u16, y: u16, width: u16, height: u16, scale: f64) -> Self {
        Self {
            area: Rect { x, y, width, height },
            scale
        }
    }

    pub fn move_to_starting_point(&self) {
        mouse::move_to(self.area.x + self.area.width / 2, self.area.y + self.area.height / 2);
    }

    pub fn draw_less_than(&self) {
        mouse::down(Button::Left);
        mouse::slow_drag_rel((-100f64 * self.scale) as i32, (50f64 * self.scale) as i32, Speed::Normal);
        mouse::slow_drag_rel((100f64 * self.scale) as i32, (50f64 * self.scale) as i32, Speed::Normal);
        mouse::up(Button::Left);
    }

    pub fn draw_greater_than(&self) {
        mouse::down(Button::Left);
        mouse::slow_drag_rel((100f64 * self.scale) as i32, (50f64 * self.scale) as i32, Speed::Normal);
        mouse::slow_drag_rel((-100f64 * self.scale) as i32, (50f64 * self.scale) as i32, Speed::Normal);
        mouse::up(Button::Left);
    }
}