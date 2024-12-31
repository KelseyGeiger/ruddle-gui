use crate::backend_sdl2::*;
use sdl2::rect::*;
use sdl2::pixels::*;
use sdl2::render::*;
use std::vec::Vec;

use super::button::*;

pub struct Panel {
    pub bounds: Rect,
    pub color: Color,
    pub buttons: Vec<Button>,
}

impl Panel {
    pub fn new(x: i32, y: i32, w: u32, h: u32, col: Color) -> Panel {
        Panel {
            bounds: Rect::new(x, y, w, h),
            color: col,
            buttons: Vec::new()
        }
    }

    pub fn with_buttons(self, btns: Vec<Button>) -> Panel {
        Panel {
            bounds: self.bounds,
            color: self.color,
            buttons: btns
        }
    }

    pub fn draw<T: RenderTarget>(&self, canvas: &mut sdl2::render::Canvas<T>) {
        draw_filled_rect(canvas, self.bounds, self.color);

        for b in &self.buttons {
            // Assume relative-offset coordinates within a panel
            let button_rect = Rect::new(self.bounds.x + b.bounds.x, self.bounds.y + b.bounds.y, b.bounds.width(), b.bounds.height());
            let button_color = if b.is_mouse_down() { b.mouse_down_color } else if b.is_mouse_over() { b.mouse_over_color } else { b.color };

            draw_filled_rect(canvas, button_rect, button_color);
        }
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        self.bounds.contains_point(Point::new(x, y))
    }

    pub fn global_to_local(&self, x: i32, y: i32) -> (i32, i32) {
        (x - self.bounds.x, y - self.bounds.y)
    }

    pub fn local_to_global(&self, x: i32, y: i32) -> (i32, i32) {
        (x + self.bounds.x, y + self.bounds.y)
    }
}