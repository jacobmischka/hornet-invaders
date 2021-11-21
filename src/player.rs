use super::*;

#[derive(Debug)]
pub struct Player {
    pub hp: i32,
    pub max_hp: i32,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub speed: f32,
    pub max_projectiles: usize,
    pub damage: u32,
    pub state: PlayerState,
    pub direction: Direction,
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
        let diameter = self.radius * 2.0;
        Rect {
            x: self.x - self.radius,
            y: self.y - self.radius,
            w: diameter,
            h: diameter,
        }
    }
}

impl Positioned for Player {
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
}

impl Mobile for Player {
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

    pub fn handle_input(&mut self) -> Vec2 {
        let mut dx = 0.0;
        let mut dy = 0.0;
        if is_key_down(KeyCode::W) {
            dy -= self.speed;
            self.direction = Direction::Up;
        }
        if is_key_down(KeyCode::S) {
            dy += self.speed;
            self.direction = Direction::Down;
        }
        if is_key_down(KeyCode::D) {
            dx += self.speed;
            self.direction = Direction::Right;
        }
        if is_key_down(KeyCode::A) {
            dx -= self.speed;
            self.direction = Direction::Left;
        }

        Vec2::new(dx, dy)
    }

    pub fn shoot(&self, direction: Direction) -> Projectile {
        let projectile_speed = 10.0;

        Projectile {
            active: true,
            x: self.x,
            y: self.y,
            radius: 20.0,
            damage: self.damage,
            velocity: direction.unit_vec() * projectile_speed,
            direction,
        }
    }
}

impl Drawable for Player {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.state.into());
        match self.direction {
            Direction::Up => {
                // head
                draw_circle(
                    self.x,
                    self.y - self.radius,
                    self.radius * 0.6,
                    self.state.into(),
                );

                // stripe
                draw_rectangle(
                    self.x - self.radius,
                    self.y - self.radius * 0.1,
                    self.radius * 2.0,
                    self.radius * 0.2,
                    BLACK,
                );

                // wings
                draw_circle(self.x - self.radius, self.y, self.radius * 0.5, WHITE);
                draw_circle(self.x + self.radius, self.y, self.radius * 0.5, WHITE);
            }
            Direction::Down => {
                // head
                draw_circle(
                    self.x,
                    self.y + self.radius,
                    self.radius * 0.6,
                    self.state.into(),
                );

                // stripe
                draw_rectangle(
                    self.x - self.radius,
                    self.y - self.radius * 0.1,
                    self.radius * 2.0,
                    self.radius * 0.2,
                    BLACK,
                );

                // wings
                draw_circle(self.x - self.radius, self.y, self.radius * 0.5, WHITE);
                draw_circle(self.x + self.radius, self.y, self.radius * 0.5, WHITE);
            }
            Direction::Left => {
                // head
                draw_circle(
                    self.x - self.radius,
                    self.y,
                    self.radius * 0.6,
                    self.state.into(),
                );

                // stripe
                draw_rectangle(
                    self.x - self.radius * 0.1,
                    self.y - self.radius,
                    self.radius * 0.2,
                    self.radius * 2.0,
                    BLACK,
                );

                // wings
                draw_circle(self.x, self.y - self.radius, self.radius * 0.5, WHITE);
                draw_circle(self.x, self.y + self.radius, self.radius * 0.5, WHITE);
            }
            Direction::Right => {
                // head
                draw_circle(
                    self.x + self.radius,
                    self.y,
                    self.radius * 0.6,
                    self.state.into(),
                );

                // stripe
                draw_rectangle(
                    self.x - self.radius * 0.1,
                    self.y - self.radius,
                    self.radius * 0.2,
                    self.radius * 2.0,
                    BLACK,
                );

                // wings
                draw_circle(self.x, self.y - self.radius, self.radius * 0.5, WHITE);
                draw_circle(self.x, self.y + self.radius, self.radius * 0.5, WHITE);
            }
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        let radius = 25.0;
        let speed = 10.0;
        let max_hp = 3;

        Player {
            hp: max_hp,
            max_hp,
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
            radius,
            speed,
            damage: 25,
            max_projectiles: 20,
            state: PlayerState::Ok,
            direction: Direction::Up,
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
            PlayerState::Ok => YELLOW,
            PlayerState::Invulnerable(_) => GRAY,
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
    pub direction: Direction,
    pub velocity: Vec2,
}

impl Actor for Projectile {
    fn tick(&mut self) {
        self.x += self.velocity[0];
        self.y += self.velocity[1];
    }

    fn bounding_box(&self) -> Rect {
        let diameter = self.radius * 2.0;
        Rect {
            x: self.x - self.radius,
            y: self.y - self.radius,
            w: diameter,
            h: diameter,
        }
    }
}

impl Drawable for Projectile {
    fn draw(&self) {
        match self.direction {
            Direction::Up => {
                draw_triangle(
                    Vec2::new(self.x, self.y - self.radius),
                    Vec2::new(self.x - self.radius / 2.0, self.y + self.radius),
                    Vec2::new(self.x + self.radius / 2.0, self.y + self.radius),
                    WHITE,
                );
            }
            Direction::Down => {
                draw_triangle(
                    Vec2::new(self.x, self.y + self.radius),
                    Vec2::new(self.x - self.radius / 2.0, self.y - self.radius),
                    Vec2::new(self.x + self.radius / 2.0, self.y - self.radius),
                    WHITE,
                );
            }
            Direction::Right => {
                draw_triangle(
                    Vec2::new(self.x + self.radius, self.y),
                    Vec2::new(self.x - self.radius, self.y + self.radius / 2.0),
                    Vec2::new(self.x - self.radius, self.y - self.radius / 2.0),
                    WHITE,
                );
            }
            Direction::Left => {
                draw_triangle(
                    Vec2::new(self.x - self.radius, self.y),
                    Vec2::new(self.x + self.radius, self.y + self.radius / 2.0),
                    Vec2::new(self.x + self.radius, self.y - self.radius / 2.0),
                    WHITE,
                );
            }
        }
    }
}
