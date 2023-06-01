extern crate sdl2;

mod gui_context;
mod element_id;
mod gui_tree_node;
mod bounds;
mod color;
mod backend_sdl2;

pub use element_id::*;
pub use gui_context::*;
pub use gui_tree_node::*;
pub use bounds::*;
use color::*;
use backend_sdl2::*;

use sdl2::pixels;
use sdl2::rect::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Clone, Debug)]
struct GuiRegion {
    top_bound: f32,
    bottom_bound: f32,
    left_bound: f32,
    right_bound: f32,
	color: pixels::Color,
    child_regions: Option<Vec<GuiRegion>>,
}

impl GuiRegion {
	pub fn new(left: f32, right: f32, bottom: f32, top: f32, col: pixels::Color) -> GuiRegion {
		let (bottom, top) = if top > bottom {
			(top, bottom)
		} else {
			(bottom, top)
		};
		
		let (left, right) = if left > right {
			(right, left)
		} else {
			(left, right)
		};
		
		GuiRegion {
			top_bound: top,
			bottom_bound: bottom,
			left_bound: left,
			right_bound: right,
			color: col,
			child_regions: None
		}
	}
	pub fn from_rect(x: f32, y: f32, w: f32, h: f32, col: pixels::Color) -> GuiRegion {
		let (x, w) = if w < 0.0f32 {
			(x + w, -w)
		} else {
			(x, w)
		};
		
		let (y, h) = if h < 0.0f32 {
			(y + h, -h)
		} else {
			(y, h)
		};
		
		GuiRegion {
			bottom_bound: y + h,
			top_bound: y,
			left_bound: x,
			right_bound: x + w,
			color: col,
			child_regions: None,
		}
	}
	
	pub fn split_at_relative_x(&mut self, x: f32, left_color: pixels::Color, right_color: pixels::Color) {
		match &mut self.child_regions {
			None => {
				if x > 0.0f32 && x <= (self.right_bound - self.left_bound) {
					let split_x = self.left_bound + x;
					let left_child_left = self.left_bound;
					let right_child_right = self.right_bound;
					
					let left_color = blend_colors(self.color, left_color, 0.5f32);
					let right_color = blend_colors(self.color, right_color, 0.5f32);
					
					let left_child = GuiRegion::new(left_child_left, split_x, self.bottom_bound, self.top_bound, left_color);
					let right_child = GuiRegion::new(split_x, right_child_right, self.bottom_bound, self.top_bound, right_color);
					
					self.child_regions = Some(vec![left_child, right_child]);
				}
			},
			Some(children) =>
			{
				if x > 0.0f32 && x <= (self.right_bound - self.left_bound) {
					let absolute_x = self.left_bound + x;
					for i in 0..children.len() {
						if children[i].left_bound < absolute_x && children[i].right_bound > absolute_x {
							let child_relative_x = absolute_x - children[i].left_bound;
							
							let left_color = blend_colors(self.color, left_color, 0.5f32);
							let right_color = blend_colors(self.color, right_color, 0.5f32);
							
							children[i].split_at_relative_x(child_relative_x, left_color, right_color);
						}
					}
				}
			}
		}
	}
	
	pub fn split_at_relative_y(&mut self, y: f32, top_color: pixels::Color, bottom_color: pixels::Color) {
		match &mut self.child_regions {
			None => {
				if y > 0.0f32 && y <= (self.bottom_bound - self.top_bound) {
					let split_y = self.bottom_bound - y;
					let top_child_top = self.top_bound;
					let bottom_child_bottom = self.bottom_bound;
					
					let top_color = blend_colors(self.color, top_color, 0.5f32);
					let bottom_color = blend_colors(self.color, bottom_color, 0.5f32);
					
					let top_child = GuiRegion::new(self.left_bound, self.right_bound, split_y, top_child_top, top_color);
					let bottom_child = GuiRegion::new(self.left_bound, self.right_bound, bottom_child_bottom, split_y, bottom_color);
					
					self.child_regions = Some(vec![top_child, bottom_child]);
				}
			},
			Some(children) =>
			{
				if y > 0.0f32 && y <= (self.bottom_bound - self.top_bound) {
					let absolute_y = self.top_bound + y;
					for i in 0..children.len() {
						if children[i].top_bound < absolute_y && children[i].bottom_bound > absolute_y {
							let child_relative_y = absolute_y - children[i].top_bound;
							
							let top_color = blend_colors(self.color, top_color, 0.5f32);
							let bottom_color = blend_colors(self.color, bottom_color, 0.5f32);
					
							children[i].split_at_relative_y(child_relative_y, top_color, bottom_color);
						}
					}
				}
			}
		}
	}
	
	pub fn split_at_relative_point(&mut self, x: f32, y: f32, top_left_color: pixels::Color, top_right_color: pixels::Color, bottom_left_color: pixels::Color, bottom_right_color: pixels::Color) {
		match &mut self.child_regions {
			None => {
				if x > 0.0f32 && x <= (self.right_bound - self.left_bound) && 
				   y > 0.0f32 && y <= (self.bottom_bound - self.top_bound)
				{
					let split_x = self.left_bound + x;
					let left_child_left = self.left_bound;
					let right_child_right = self.right_bound;
					
					let split_y = self.top_bound + y;
					let top_child_top = self.top_bound;
					let bottom_child_bottom = self.bottom_bound;
					
					let top_left_color = blend_colors(self.color, top_left_color, 0.5f32);
					let top_right_color = blend_colors(self.color, top_right_color, 0.5f32);
					let bottom_left_color = blend_colors(self.color, bottom_left_color, 0.5f32);
					let bottom_right_color = blend_colors(self.color, bottom_right_color, 0.5f32);
					
					let top_left_child = GuiRegion::new(left_child_left, split_x, split_y, top_child_top, top_left_color);
					let top_right_child = GuiRegion::new(split_x, right_child_right, split_y, top_child_top, top_right_color);
					let bottom_left_child = GuiRegion::new(left_child_left, split_x, bottom_child_bottom, split_y, bottom_left_color);
					let bottom_right_child = GuiRegion::new(split_x, right_child_right, bottom_child_bottom, split_y, bottom_right_color);
					
					self.child_regions = Some(vec![top_left_child, top_right_child, bottom_left_child, bottom_right_child]);
				}
			},
			Some(children) =>
			{
				if x > 0.0f32 && x <= (self.right_bound - self.left_bound) && 
				   y > 0.0f32 && y <= (self.bottom_bound - self.top_bound) 
				{
					let absolute_x = self.left_bound + x;
					let absolute_y = self.top_bound + y;
					for i in 0..children.len() {

						if children[i].left_bound < absolute_x && children[i].right_bound > absolute_x && 
						   children[i].top_bound < absolute_y && children[i].bottom_bound > absolute_y 
						{
							let child_relative_x = absolute_x - children[i].left_bound;
							let child_relative_y = absolute_y - children[i].top_bound;
							
							let top_left_color = blend_colors(self.color, top_left_color, 0.5f32);
							let top_right_color = blend_colors(self.color, top_right_color, 0.5f32);
							let bottom_left_color = blend_colors(self.color, bottom_left_color, 0.5f32);
							let bottom_right_color = blend_colors(self.color, bottom_right_color, 0.5f32);
							
							children[i].split_at_relative_point(child_relative_x, child_relative_y, top_left_color, top_right_color, bottom_left_color, bottom_right_color);
							break;
						}
					}
				}
			}
		}
	}
	
}

fn region_to_colored_rect(region: &GuiRegion) -> (Rect, pixels::Color) {
	let width = region.right_bound - region.left_bound;
	let height = region.bottom_bound - region.top_bound;
	let rect = Rect::new(region.left_bound as i32, region.top_bound as i32, width as u32, height as u32);
	(rect, region.color)
}

fn draw_region<'a, T: sdl2::render::RenderTarget>(canvas: &mut sdl2::render::Canvas<T>, 
												  region: &'a GuiRegion, 
												  border_color: sdl2::pixels::Color, 
												  border_size: u32)
{
	let region_rect = region_to_colored_rect(region);
	let next_border_color = region_rect.1;
	draw_bordered_filled_rect(canvas, region_rect.0, border_size, region_rect.1, border_color);
	

	match &region.child_regions {
		Some(children) => {
			for i in 0..children.len() {
				let next_border_size = if border_size > 1 { border_size / 2 } else { border_size };
				draw_region(canvas, &children[i], next_border_color, next_border_size);
			}
		},
		None => {}
	}
}

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ruddle-gui", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(232, 230, 224));
    canvas.clear();
    canvas.present();
	
	let mut color_idx: usize = 0;
	let colors = [pixels::Color::WHITE, pixels::Color::BLACK, pixels::Color::RED, pixels::Color::GREEN, pixels::Color::BLUE, pixels::Color::GRAY, pixels::Color::CYAN, pixels::Color::YELLOW, pixels::Color::MAGENTA];
	let mut next_color = || { let col = colors[color_idx]; color_idx = (color_idx + 1) % colors.len(); return col; };

	let mut window_region = GuiRegion::from_rect(0.0f32, 0.0f32, 800.0f32, 600.0f32, next_color());
	window_region.split_at_relative_x(400.0f32, next_color(), next_color());
	window_region.split_at_relative_y(200.0f32, next_color(), next_color());
	window_region.split_at_relative_point(200.0f32, 200.0f32, next_color(), next_color(), next_color(), next_color());
	window_region.split_at_relative_point(600.0f32, 500.0f32, next_color(), next_color(), next_color(), next_color());

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(pixels::Color::RGB(232, 230, 224));
        canvas.clear();

        draw_region(&mut canvas, &window_region, pixels::Color::RGB(232, 230, 224), 32);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {

                },
                Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {

                },
                _ => {}
            }
        }

        canvas.present();
    }
}
