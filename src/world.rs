use super::*;

#[derive(Debug, Default)]
pub struct World {
    state: GameState,
    player: Player,
    stage: usize,
    hives_saved: usize,
    projectiles: Vec<Projectile>,
    enemies: Vec<Enemy>,
    enemies_remaining: usize,
    terrain: Vec<Terrain>,
    hives: Vec<Hive>,
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

        for h in &self.hives {
            h.draw();
        }

        self.player.draw();

        for projectile in &self.projectiles {
            projectile.draw();
        }

        if self.state == GameState::Victory {
            draw_centered_text(
                "YOU WIN",
                screen_width() / 2.0,
                screen_height() / 2.0,
                100,
                WHITE,
            );

            draw_centered_text(
                "Press ENTER to progress",
                screen_width() / 2.0,
                screen_height() / 2.0 + 100.0,
                50,
                WHITE,
            );
        } else if self.state == GameState::Defeat {
            draw_centered_text(
                "GAME OVER",
                screen_width() / 2.0,
                screen_height() / 2.0,
                100,
                RED,
            );
            draw_centered_text(
                "Press ENTER to try again",
                screen_width() / 2.0,
                screen_height() / 2.0 + 100.0,
                50,
                WHITE,
            );
        }

        self.draw_hud();
    }
}

impl World {
    pub fn reset(&mut self) {
        self.stage = 0;
        self.hives_saved = 0;
        self.set_stage();
    }

    pub fn set_stage(&mut self) {
        self.terrain = (0..rand::gen_range(3, 10 + self.stage))
            .map(|_| {
                let mut f = Terrain::default();
                f.kind = TerrainKind::Flower;
                f
            })
            .collect();
        self.hives = (0..3).map(|_| Hive::default()).collect();
        self.enemies_remaining = (self.stage + 1) * 10;
        self.state = GameState::Game;
        self.player = Player::default();
        self.enemies.clear();
        self.projectiles.clear();
    }

    pub fn max_enemies(&self) -> usize {
        (self.stage + 1) * 5
    }

    pub fn stage_speed(&self) -> f32 {
        self.stage as f32 * 0.5 + 1.0
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
                    self.stage += 1;
                    self.hives_saved += self.hives.len();
                    self.set_stage();
                }
            }
            GameState::Game => {
                self.handle_input();
                self.player.tick();

                if self.enemies.len() < self.max_enemies() && self.enemies_remaining > 0 {
                    self.enemies.push(Enemy::with_speed(self.stage_speed()));
                    self.enemies_remaining -= 1;
                }

                for projectile in &mut self.projectiles {
                    projectile.tick();

                    for enemy in &mut self.enemies {
                        if enemy.collides_with(projectile) {
                            enemy.hp -= projectile.damage as i32;
                            projectile.active = false;
                        }
                    }

                    for terrain in &mut self.terrain {
                        if projectile.collides_with(terrain) {
                            projectile.active = false;
                        }
                    }

                    if projectile.fully_offscreen() {
                        projectile.active = false;
                    }
                }

                for enemy in &mut self.enemies {
                    let mut desired_movement = enemy.desired_movement(&self.hives);
                    for t in &mut self.terrain {
                        desired_movement = enemy.handle_collision(desired_movement, t);
                    }
                    enemy.move_by(desired_movement);
                    enemy.tick();

                    for hive in &mut self.hives {
                        if enemy.collides_with(hive) {
                            hive.hp -= 1;
                            enemy.hp = 0;
                            break;
                        }
                    }

                    if self.player.state == PlayerState::Ok {
                        if enemy.collides_with(&self.player) {
                            self.player.hp -= 1;
                            self.player.state = PlayerState::Invulnerable(get_time() + 1.0);
                        }
                    }
                }

                self.projectiles.retain(|projectile| projectile.active);
                self.enemies.retain(|enemy| enemy.hp > 0);
                self.hives.retain(|hive| hive.hp > 0);

                if self.player.hp <= 0 || self.hives.is_empty() {
                    self.state = GameState::Defeat;
                    return;
                }

                if self.enemies.is_empty() {
                    self.state = GameState::Victory;
                    return;
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

    fn draw_hud(&self) {
        self.player.draw_hp();
        draw_text(
            &format!("Hives saved: {}", self.hives_saved),
            20.0,
            50.0,
            50.0,
            LIGHTGRAY,
        );
        draw_h_centered_text(
            &format!("Stage: {}", self.stage + 1),
            screen_width() / 2.0,
            50.0,
            50,
            LIGHTGRAY,
        );
        draw_text(&get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);
    }
}
