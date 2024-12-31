use crate::backend_sdl2::*;
use sdl2::rect::*;
use sdl2::pixels::*;
use sdl2::render::*;

pub struct Button {
    pub bounds: Rect,
    mouse_over: bool,
    mouse_down: bool,
    pub color: Color,
    pub mouse_over_color: Color,
    pub mouse_down_color: Color,
    onclick_action: Box<dyn Fn() -> ()>
}

impl Button {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: Color, onclick: &'static dyn Fn() -> ()) -> Button
    {
        Button {
            bounds: Rect::new(x, y, w, h),
            mouse_over: false,
            mouse_down: false,
            color: color,
            mouse_over_color: blend_colors(color, Color::WHITE, 0.1),
            mouse_down_color: blend_colors(color, Color::BLACK, 0.1),
            onclick_action: Box::new(onclick)
        }
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool
    {
        self.bounds.contains_point(Point::new(x, y))
    }

    pub fn draw<T: RenderTarget>(&self, canvas: &mut sdl2::render::Canvas<T>)
    {
        let draw_color = if self.mouse_down { self.mouse_down_color } else if self.mouse_over { self.mouse_over_color } else { self.color };
        draw_filled_rect(canvas, self.bounds, draw_color);
    }

    pub fn is_mouse_over(&self) -> bool
    {
        self.mouse_over
    }

    pub fn is_mouse_down(&self) -> bool
    {
        self.mouse_down
    }

    pub fn on_mouse_over(&mut self)
    {
        self.mouse_over = true;
    }

    pub fn on_mouse_leave(&mut self)
    {
        self.mouse_over = false;
        self.mouse_down = false;
    }

    pub fn on_mouse_down(&mut self)
    {
        if self.mouse_over {
            self.mouse_down = true;
        }
    }

    pub fn on_mouse_up(&mut self)
    {
        if self.mouse_down
        {
            (self.onclick_action)()
        }
        self.mouse_down = false;
    }
}