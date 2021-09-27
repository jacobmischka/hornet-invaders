use super::*;

#[derive(Debug)]
pub struct Player {
    pub hp: i32,
    pub max_hp: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub max_projectiles: usize,
    pub damage: u32,
    pub state: PlayerState,
}

impl Actor for Player {
    fn tick(&mut self) {
        if let PlayerState::Invulnerable(until) = self.state {
            if until < get_time() {
                self.state = PlayerState::Ok;
            }
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

impl Mobile for Player {
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

impl Player {
    pub fn draw_hp(&self) {
        draw_right_aligned_text(
            &format!("HP: {} / {}", self.hp, self.max_hp),
            50,
            screen_width() - 20.0,
            50.0,
            LIGHTGRAY,
        );
    }

    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::W) {
            self.y -= self.speed;
        }
        if is_key_down(KeyCode::S) {
            self.y += self.speed;
        }
        if is_key_down(KeyCode::D) {
            self.x += self.speed;
        }
        if is_key_down(KeyCode::A) {
            self.x -= self.speed;
        }

        self.screen_constrain();
    }

    pub fn shoot(&self, direction: Direction) -> Projectile {
        Projectile {
            active: true,
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
            radius: 5.0,
            damage: self.damage,
            direction: direction.unit_vec() * 10.0,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn unit_vec(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        }
    }
}

impl Drawable for Player {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.state.into());
    }
}

impl Default for Player {
    fn default() -> Self {
        let width = 25.0;
        let height = 25.0;
        let speed = 10.0;
        let max_hp = 3;

        Player {
            hp: max_hp,
            max_hp,
            x: screen_width() / 2.0 - width / 2.0,
            y: screen_height() / 2.0 - height / 2.0,
            width,
            height,
            speed,
            damage: 25,
            max_projectiles: 20,
            state: PlayerState::Ok,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerState {
    Ok,
    Invulnerable(f64),
}

impl From<PlayerState> for Color {
    fn from(state: PlayerState) -> Color {
        match state {
            PlayerState::Ok => BLUE,
            PlayerState::Invulnerable(_) => ORANGE,
        }
    }
}

#[derive(Debug)]
pub struct Projectile {
    pub active: bool,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub damage: u32,
    pub direction: Vec2,
}

impl Actor for Projectile {
    fn tick(&mut self) {
        self.x += self.direction[0];
        self.y += self.direction[1];
    }

    fn bounding_box(&self) -> Rect {
        let wh = self.radius * 2.0;
        Rect {
            x: self.x - self.radius,
            y: self.y - self.radius,
            w: wh,
            h: wh,
        }
    }
}

impl Drawable for Projectile {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, YELLOW);
    }
}
