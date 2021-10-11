use super::*;

#[derive(Debug, Default)]
pub struct World {
    state: GameState,
    player: Player,
    stage: Stage,
    projectiles: Vec<Projectile>,
    enemies: Vec<Enemy>,
    terrain: Vec<Terrain>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Test,
}

impl Default for Stage {
    fn default() -> Self {
        Stage::Test
    }
}

impl Drawable for World {
    fn draw(&self) {
        clear_background(BLACK);

        for enemy in &self.enemies {
            enemy.draw();
        }

        for t in &self.terrain {
            t.draw();
        }

        self.player.draw();

        for projectile in &self.projectiles {
            projectile.draw();
        }

        if self.state == GameState::Victory {
            let text = "YOU WIN";
            let font_size = 100;
            let size = measure_text(text, None, font_size, 1.0);

            draw_text(
                text,
                screen_width() / 2.0 - size.width / 2.0,
                screen_height() / 3.0 - size.height / 2.0,
                font_size as f32,
                WHITE,
            );
        } else if self.state == GameState::Defeat {
            draw_centered_text(
                "YOU DIED",
                100,
                screen_width() / 2.0,
                screen_height() / 2.0,
                RED,
            );
        }

        self.player.draw_hp();
    }
}

impl World {
    pub fn set_stage(&mut self, stage: Stage) {
        match stage {
            Stage::Test => {
                self.enemies = (0..5).map(|_| Enemy::default()).collect();
                self.terrain = (0..5)
                    .map(|_| Terrain::default())
                    .chain((0..3).map(|_| {
                        let mut f = Terrain::default();
                        f.kind = TerrainKind::Flower;
                        f
                    }))
                    .collect();
            }
        }

        self.state = GameState::Game;
        self.player = Player::default();
        self.projectiles.clear();
        self.stage = stage;
    }

    pub fn reset(&mut self) {
        self.set_stage(self.stage)
    }

    pub fn tick(&mut self) {
        match self.state {
            GameState::Defeat => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset();
                }
            }
            GameState::Victory => {
                if is_key_pressed(KeyCode::Enter) {
                    self.reset();
                }
            }
            GameState::Game => {
                self.handle_input();
                self.player.tick();

                for projectile in &mut self.projectiles {
                    projectile.tick();

                    for enemy in &mut self.enemies {
                        if enemy.collides_with(projectile) {
                            enemy.hp -= projectile.damage as i32;
                            projectile.active = false;
                        }
                    }

                    if projectile.fully_offscreen() {
                        projectile.active = false;
                    }
                }

                for enemy in &mut self.enemies {
                    let mut desired_movement = enemy.desired_movement();
                    for t in &self.terrain {
                        desired_movement = enemy.handle_collision(desired_movement, t);
                    }
                    enemy.move_by(desired_movement);
                    enemy.tick();

                    if self.player.state == PlayerState::Ok {
                        if enemy.collides_with(&self.player) {
                            self.player.hp -= 1;
                            self.player.state = PlayerState::Invulnerable(get_time() + 1.0);
                        }
                    }
                }

                self.projectiles.retain(|projectile| projectile.active);
                self.enemies.retain(|enemy| enemy.hp > 0);

                if self.player.hp <= 0 {
                    self.state = GameState::Defeat;
                }

                if self.enemies.is_empty() {
                    self.state = GameState::Victory;
                }
            }
        }
    }

    fn handle_input(&mut self) {
        let mut player_movement = self.player.handle_input();

        for t in &self.terrain {
            player_movement = self.player.handle_collision(player_movement, t);
        }

        self.player.move_by(player_movement);
        self.player.screen_constrain();

        if self.projectiles.len() < self.player.max_projectiles {
            if is_key_pressed(KeyCode::Up) {
                self.projectiles.push(self.player.shoot(Direction::Up));
            }
            if is_key_pressed(KeyCode::Down) {
                self.projectiles.push(self.player.shoot(Direction::Down));
            }
            if is_key_pressed(KeyCode::Left) {
                self.projectiles.push(self.player.shoot(Direction::Left));
            }
            if is_key_pressed(KeyCode::Right) {
                self.projectiles.push(self.player.shoot(Direction::Right));
            }
        }
    }
}
