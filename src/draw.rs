use macroquad::prelude::*;

pub fn draw_centered_text(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let size = measure_text(text, None, font_size, 1.0);
    draw_text(
        text,
        x - size.width / 2.0,
        y - size.height / 2.0,
        font_size as f32,
        color,
    );
}

pub fn draw_v_centered_text(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let size = measure_text(text, None, font_size, 1.0);
    draw_text(text, x, y - size.height / 2.0, font_size as f32, color);
}

pub fn draw_h_centered_text(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let size = measure_text(text, None, font_size, 1.0);
    draw_text(text, x - size.width / 2.0, y, font_size as f32, color);
}

pub fn draw_right_aligned_text(text: &str, x: f32, y: f32, font_size: u16, color: Color) {
    let size = measure_text(text, None, font_size, 1.0);
    draw_text(text, x - size.width, y, font_size as f32, color);
}
