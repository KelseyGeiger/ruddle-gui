extern crate sdl2;

mod gui_context;
mod element_id;
mod gui_tree_node;
mod bounds;
mod color;
mod backend_sdl2;

mod gui_prototype;

pub use element_id::*;
pub use gui_context::*;
pub use gui_tree_node::*;
pub use bounds::*;
use color::*;
use backend_sdl2::*;

use crate::gui_prototype as proto;
use proto::{Button, Panel};

use sdl2::pixels;
use sdl2::rect::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::RenderTarget;

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
	let colors = [pixels::Color::RED, pixels::Color::GREEN, pixels::Color::BLUE, pixels::Color::GRAY, pixels::Color::CYAN, pixels::Color::YELLOW, pixels::Color::MAGENTA];
	let mut next_color = || { let col = colors[color_idx]; color_idx = (color_idx + 1) % colors.len(); return col; };

    let mut buttons: Vec<Button> = Vec::new();
    buttons.push(Button::new(50, 50, 60, 40, colors[0], &|| { println!("First button clicked!"); }));
    buttons.push(Button::new(50, 120, 60, 40, colors[1], &|| { println!("Second button clicked!"); }));
    buttons.push(Button::new(50, 190, 60, 40, colors[2], &|| {println!("Third button clicked!"); }));

    let mut test_panel: Panel = Panel::new(150, 210, 160, 300, colors[3]).with_buttons(buttons);

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(pixels::Color::RGB(232, 230, 224));
        canvas.clear();

        test_panel.draw(&mut canvas);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                    if test_panel.contains_point(x, y) {
                        let (local_x, local_y) = test_panel.global_to_local(x, y);

                        for button in &mut test_panel.buttons {
                            if button.contains_point(local_x, local_y) {
                                button.on_mouse_over();
                            }
                            else {
                                if button.is_mouse_over() {
                                    button.on_mouse_leave();
                                }
                            }
                        }
                    }
                }
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {
                    for button in &mut test_panel.buttons {
                        button.on_mouse_down();
                    }
                },
                Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {
                    for button in &mut test_panel.buttons {
                        button.on_mouse_up();
                    }
                },
                _ => {}
            }
        }

        canvas.present();
    }
}
