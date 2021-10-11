use macroquad::prelude::*;

use std::{convert::From, default::Default, ops::Neg};

mod draw;
mod enemy;
mod player;
mod terrain;
mod world;

use draw::*;
use enemy::*;
use player::*;
use terrain::*;
use world::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Hello, Macroquad!".to_owned(),
        high_dpi: true,
        window_width: 1600,
        window_height: 1200,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::default();
    world.reset();

    loop {
        world.tick();
        world.draw();
        draw_fps();

        next_frame().await
    }
}

fn draw_fps() {
    draw_text(&get_fps().to_string(), 20.0, 20.0, 30.0, DARKGRAY);
}

trait Drawable {
    fn draw(&self);
}

trait Mobile: Actor {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn move_by(&mut self, vector: Vec2);

    fn screen_constrain(&mut self) {
        let bb = self.bounding_box();
        let screen_w = screen_width();
        let x = if bb.x < 0.0 {
            bb.x * -1.0
        } else if bb.right() > screen_w {
            screen_w - (bb.x + bb.w)
        } else {
            0.0
        };

        let screen_h = screen_height();
        let y = if bb.y < 0.0 {
            bb.y * -1.0
        } else if bb.bottom() > screen_h {
            screen_h - (bb.y + bb.h)
        } else {
            0.0
        };

        self.move_by(Vec2::new(x, y));
    }

    fn handle_collision(&mut self, desired_movement: Vec2, collider: &impl Actor) -> Vec2 {
        let mut x = desired_movement * Vec2::X;
        let mut y = desired_movement * Vec2::Y;

        let my_bb = self.bounding_box();
        let x_bb = my_bb.offset(x);
        let y_bb = my_bb.offset(y);

        let collider_bb = collider.bounding_box();
        if x.length() > 0.0 {
            if let Some(intersection) = my_bb.offset(x).intersect(collider_bb) {
                if intersection.h > 0.0 {
                    x += Vec2::new(
                        if intersection.left() == x_bb.left() {
                            intersection.w
                        } else {
                            intersection.w.neg()
                        },
                        0.0,
                    );
                }
            }
        }

        if y.length() > 0.0 {
            if let Some(intersection) = y_bb.intersect(collider_bb) {
                if intersection.w > 0.0 {
                    y += Vec2::new(
                        0.0,
                        if intersection.top() == y_bb.top() {
                            intersection.h
                        } else {
                            intersection.h.neg()
                        },
                    );
                }
            }
        }

        x + y
    }
}

trait Actor: Drawable + std::fmt::Debug {
    fn tick(&mut self);

    fn bounding_box(&self) -> Rect;

    fn collides_with(&self, other: &dyn Actor) -> bool {
        self.bounding_box().overlaps(&other.bounding_box())
    }

    fn fully_onscreen(&self) -> bool {
        let bb = self.bounding_box();
        bb.x > 0.0
            && (bb.x + bb.w) < screen_width()
            && bb.y > 0.0
            && (bb.y + bb.h) < screen_height()
    }

    fn fully_offscreen(&self) -> bool {
        let bb = self.bounding_box();
        bb.x > screen_width()
            || (bb.x + bb.w) < 0.0
            || bb.y > screen_height()
            || (bb.y + bb.h) < 0.0
    }
}

impl<D: Drawable> Drawable for Box<D> {
    #[inline]
    fn draw(&self) {
        (**self).draw();
    }
}

impl<A: Actor + Drawable> Actor for Box<A> {
    #[inline]
    fn tick(&mut self) {
        (**self).tick();
    }
    #[inline]
    fn bounding_box(&self) -> Rect {
        (**self).bounding_box()
    }
    #[inline]
    fn collides_with(&self, other: &dyn Actor) -> bool {
        (**self).collides_with(other)
    }
    #[inline]
    fn fully_onscreen(&self) -> bool {
        (**self).fully_onscreen()
    }
    #[inline]
    fn fully_offscreen(&self) -> bool {
        (**self).fully_offscreen()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Game,
    Victory,
    Defeat,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Game
    }
}
