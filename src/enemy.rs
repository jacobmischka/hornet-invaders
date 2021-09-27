use super::*;

#[derive(Debug)]
pub struct Enemy {
    pub hp: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Enemy {
        let width = 50.0;
        let height = 50.0;

        Enemy {
            hp: 50,
            x: rand::gen_range(0.0, screen_width() - width),
            y: rand::gen_range(0.0, screen_height() - height),
            width,
            height,
            speed: 5.0,
        }
    }
}

impl Actor for Enemy {
    fn tick(&mut self) {
        let d = Vec2::new(
            rand::gen_range(-1.0 * self.speed, self.speed),
            rand::gen_range(-1.0 * self.speed, self.speed),
        );

        self.move_by(d);
        self.screen_constrain();
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

impl Mobile for Enemy {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn move_by(&mut self, vector: Vec2) {
        self.x += vector[0];
        self.y += vector[1];
    }
}

impl Drawable for Enemy {
    fn draw(&self) {
        draw_rectangle(
            self.x,
            self.y,
            self.width,
            self.height,
            match self.hp {
                0..=25 => ORANGE,
                _ => RED,
            },
        );
    }
}
