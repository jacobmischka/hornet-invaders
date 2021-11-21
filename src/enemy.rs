use super::*;

use macroquad::rand::ChooseRandom;

#[derive(Debug)]
pub struct Enemy {
    pub hp: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub target: Option<Vec2>,
}

impl Enemy {
    pub fn with_speed(speed: f32) -> Self {
        let width = 50.0;
        let height = 50.0;

        let dir = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .choose()
        .copied()
        .unwrap();

        Enemy {
            hp: 50,
            x: match dir {
                Direction::Up | Direction::Down => rand::gen_range(0.0, screen_width()),
                Direction::Left => 0.0 - width,
                Direction::Right => screen_width(),
            },
            y: match dir {
                Direction::Left | Direction::Right => rand::gen_range(0.0, screen_width()),
                Direction::Up => 0.0 - width,
                Direction::Down => screen_height(),
            },
            width,
            height,
            speed,
            target: None,
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
        //
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
