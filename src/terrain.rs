use super::*;

use std::f32::consts::FRAC_PI_3;

#[derive(Debug)]
pub struct Terrain {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub kind: TerrainKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainKind {
    Block,
    Flower,
    Hive { hp: i32, max_hp: i32 },
}

impl Default for Terrain {
    fn default() -> Self {
        let width = rand::gen_range(50.0, 300.0);
        let height = rand::gen_range(50.0, 300.0);

        Terrain {
            x: rand::gen_range(0.0, screen_width() - width),
            y: rand::gen_range(0.0, screen_height() - height),
            width,
            height,
            kind: Default::default(),
        }
    }
}

impl Default for TerrainKind {
    fn default() -> Self {
        TerrainKind::Block
    }
}

impl Terrain {}

impl Drawable for Terrain {
    fn draw(&self) {
        match self.kind {
            TerrainKind::Block => draw_rectangle(self.x, self.y, self.width, self.height, MAGENTA),
            TerrainKind::Flower => {
                let half_width = self.width / 2.0;
                let mid_x = self.x + half_width;
                let mid_y = self.y + half_width;

                draw_rectangle(
                    self.x + self.width / 4.0,
                    self.y + half_width,
                    half_width,
                    self.height - half_width,
                    GREEN,
                );

                let hypot = self.width / 3.0;
                let radius = self.width / 5.0;

                for i in 0..6 {
                    let t = i as f32 * FRAC_PI_3;
                    let opposite = t.sin() * hypot;
                    let adjacent = t.cos() * hypot;

                    draw_circle(mid_x + adjacent, mid_y + opposite, radius, YELLOW);
                }

                draw_circle(mid_x, mid_y, radius, WHITE);
            }
            TerrainKind::Hive { hp, max_hp } => {
                let color = Color::new(0.9, 0.9 * (hp as f32 / max_hp as f32), 0.0, 1.0);
                const NUM_CHUNKS: usize = 5;

                let chunk_height = self.height / NUM_CHUNKS as f32;
                let mut y = self.y + self.height - chunk_height;

                let mut width = self.width;
                let chunk_width_delta = (self.width * 0.75) / NUM_CHUNKS as f32;

                for _ in 0..NUM_CHUNKS {
                    let x = self.x + (self.width - width) / 2.0;
                    draw_rectangle(x, y, width, chunk_height, color);
                    let r = chunk_height / 2.0;

                    draw_circle(x, y + r, r, color);
                    draw_circle(x + width, y + r, r, color);

                    width -= chunk_width_delta;
                    y -= chunk_height;
                }
            }
        }
    }
}

impl Actor for Terrain {
    fn tick(&mut self) {
        // Nothing to do
    }

    fn bounding_box(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.width,
            h: self.height,
        }
    }
}
