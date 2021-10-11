use super::*;

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
                draw_rectangle(
                    self.x + self.width * 0.25,
                    self.y + half_width,
                    half_width,
                    self.height - half_width,
                    GREEN,
                );
                draw_circle(self.x + half_width, self.y + half_width, half_width, YELLOW);
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
