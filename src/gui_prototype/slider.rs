use crate::backend_sdl2::*;
use sdl2::rect::*;
use sdl2::pixels::*;
use sdl2::render::*;

use std::default;
use std::ops::*;
use std::cell::*;

pub struct Slider<T>
{
    pub start_pos: Point,
    pub end_pos: Point,
    value_bounds: (T, T),
    value: Cell<T>,
}

impl<T> Slider<T>
    where T: Sized + Add<T> + Mul<T> + Sub<T> + Div<T> + PartialOrd + PartialEq + Copy
{
    pub fn new(start: (i32, i32), end: (i32, i32), lower_bound: T, upper_bound: T, default_value: Option<T>) -> Slider<T>
    {
        Slider {
            start_pos: Point::new(start.0, start.1),
            end_pos: Point::new(end.0, end.1),
            value_bounds: if lower_bound > upper_bound { (upper_bound, lower_bound) } else { (lower_bound, upper_bound) },
            value: Cell::new(if let Some(val) = default_value { val } else { lower_bound })
        }
    }
}