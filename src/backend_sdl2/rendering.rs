use sdl2::rect::*;
use sdl2::pixels::*;
use sdl2::render::*;

pub fn draw_filled_rect<T: RenderTarget>(canvas: &mut Canvas<T>, rect: Rect, color: Color)
{
    let original_color = canvas.draw_color();
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
    canvas.set_draw_color(original_color);
}

pub fn draw_bordered_filled_rect<T: RenderTarget>(canvas: &mut Canvas<T>, rect: Rect, border: u32, color: Color, border_color: Color) 
{
    let original_color = canvas.draw_color();
    let border_rect = Rect::new(rect.x() - (border as i32) / 2,
                                rect.y() - (border as i32) / 2,
                                rect.width() + border,
                                rect.height() + border);

    let adjusted_width = if rect.width() > border { rect.width() - border } else { 1 };
    let adjusted_height = if rect.height() > border { rect.height() - border } else { 1 };

	let adjusted_rect = Rect::new(rect.x() + (border as i32) / 2,
								  rect.y() + (border as i32) / 2,
								  adjusted_width,
								  adjusted_height);
    canvas.set_draw_color(border_color);
    canvas.fill_rect(border_rect).unwrap();
    canvas.set_draw_color(color);
    canvas.fill_rect(adjusted_rect).unwrap();
    canvas.set_draw_color(original_color);
}

pub fn blend_colors(base: Color, addition: Color, weight: f32) -> Color
{
	let base_corrected = [(base.r as f32 / 255.0f32).powf(2.2f32),
						  (base.g as f32 / 255.0f32).powf(2.2f32),
						  (base.b as f32 / 255.0f32).powf(2.2f32)];
	let addition_corrected = [(addition.r as f32 / 255.0f32).powf(2.2f32),
							  (addition.g as f32 / 255.0f32).powf(2.2f32),
							  (addition.b as f32 / 255.0f32).powf(2.2f32)];
							  
	let mut final_color = [0.0f32, 0.0f32, 0.0f32];
	let mut as_rgb24 = [0u8, 0u8, 0u8];
	
	for i in 0..final_color.len() {
		final_color[i] = ((base_corrected[i] * (1.0f32 - weight) + addition_corrected[i] * weight)).powf(1.0f32/2.2f32);
		as_rgb24[i] = (final_color[i] * 255.0f32) as u8;
	}
	
	Color::RGBA(as_rgb24[0], as_rgb24[1], as_rgb24[2], 255)
}