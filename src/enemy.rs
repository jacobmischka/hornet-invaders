use super::*;

use macroquad::rand::ChooseRandom;

const IMMOBILE_VOID_TIME_SECS: f64 = 5.0;

#[derive(Debug)]
pub struct Enemy {
    pub hp: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub direction: Direction,
    pub target: Option<Vec2>,
    last_position: (f32, f32, f64),
}

impl Enemy {
    pub fn with_speed(speed: f32) -> Self {
        let direction = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .choose()
        .copied()
        .unwrap();

        let (width, height) = match direction {
            Direction::Up | Direction::Down => (25.0, 100.0),
            Direction::Left | Direction::Right => (100.0, 25.0),
        };

        let x = match direction {
            Direction::Up | Direction::Down => rand::gen_range(0.0, screen_width()),
            Direction::Left => 0.0 - width,
            Direction::Right => screen_width(),
        };

        let y = match direction {
            Direction::Left | Direction::Right => rand::gen_range(0.0, screen_width()),
            Direction::Up => 0.0 - width,
            Direction::Down => screen_height(),
        };

        Enemy {
            hp: 50,
            x,
            y,
            width,
            height,
            speed,
            direction,
            target: None,
            last_position: (x, y, get_time()),
        }
    }

    pub fn desired_movement(&mut self, hives: &Vec<Hive>) -> Vec2 {
        if self.target.is_none() || rand::gen_range(0, 100) == 0 {
            self.target = hives.choose().map(|t| t.pos());
        }

        let target = self.target.unwrap();

        Vec2::new(target.x - self.x, target.y - self.y).clamp_length(0.0, self.speed)
    }
}

impl Default for Enemy {
    fn default() -> Enemy {
        Enemy::with_speed(2.0)
    }
}

impl Actor for Enemy {
    fn tick(&mut self) {
        let (x, y, time) = self.last_position;
        if x == self.x && y == self.y && (get_time() - time) > IMMOBILE_VOID_TIME_SECS {
            self.hp = 0;
        }
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

impl Positioned for Enemy {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
}

impl Mobile for Enemy {
    fn move_by(&mut self, vector: Vec2) {
        let [dx, dy] = vector.as_ref();
        if dx.abs() >= 0.001 && dy.abs() >= 0.001 {
            self.x += *dx;
            self.y += *dy;
            self.last_position = (self.x, self.y, get_time());
        }
    }
}

impl Drawable for Enemy {
    fn draw(&self) {
        let color = match self.hp {
            0..=25 => RED,
            _ => ORANGE,
        };

        let r = self.width.min(self.height) / 2.0;

        // thorax
        match self.direction {
            Direction::Up | Direction::Down => {
                draw_rectangle(self.x, self.y + r, self.width, self.height - 2.0 * r, color);
            }
            Direction::Left | Direction::Right => {
                draw_rectangle(self.x + r, self.y, self.width - 2.0 * r, self.height, color);
            }
        }

        // stripes
        match self.direction {
            Direction::Up | Direction::Down => {
                draw_rectangle(
                    self.x,
                    self.y + self.height / 3.0 - self.height / 10.0,
                    self.width,
                    self.height / 5.0,
                    BLACK,
                );
                draw_rectangle(
                    self.x,
                    self.y + self.height * 2.0 / 3.0 - self.height / 10.0,
                    self.width,
                    self.height / 5.0,
                    BLACK,
                );
            }
            Direction::Left | Direction::Right => {
                draw_rectangle(
                    self.x + self.width / 3.0 - self.width / 10.0,
                    self.y,
                    self.width / 5.0,
                    self.height,
                    BLACK,
                );
                draw_rectangle(
                    self.x + self.width * 2.0 / 3.0 - self.width / 10.0,
                    self.y,
                    self.width / 5.0,
                    self.height,
                    BLACK,
                );
            }
        }

        // ends
        match self.direction {
            Direction::Up | Direction::Down => {
                draw_circle(self.x + r, self.y + r, r, color);
                draw_circle(self.x + r, self.y + self.height - r, r, color);
            }
            Direction::Left | Direction::Right => {
                draw_circle(self.x + r, self.y + r, r, color);
                draw_circle(self.x + self.width - r, self.y + r, r, color);
            }
        }

        // wings
        match self.direction {
            Direction::Up | Direction::Down => {
                let y = self.y + self.height / 2.0;
                draw_circle(self.x - r, y, r, LIGHTGRAY);
                draw_circle(self.x + self.width + r, y, r, LIGHTGRAY);
            }
            Direction::Left | Direction::Right => {
                let x = self.x + self.width / 2.0;
                draw_circle(x, self.y - r, r, LIGHTGRAY);
                draw_circle(x, self.y + self.height + r, r, LIGHTGRAY);
            }
        }
    }
}
