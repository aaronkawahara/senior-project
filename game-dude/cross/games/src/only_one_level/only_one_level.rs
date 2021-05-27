use crate::collisions::{BoundingBox, Collideable};
use crate::common::{Position, Velocity};
use crate::images::{OnlyOneLevelPlayerBackgroundImage, OnlyOneLevelPlayerImage, SimpleImage};
use crate::rng;

use super::environment;
use super::levels::*;

use board::input::Inputs;
use defmt;
use lcd;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn play(input: &mut Inputs, dma2d: &mut Dma2d, draw_and_wait: fn() -> ()) -> u32 {
    let mut level: &dyn LevelBehavior = &LevelOne;
    let mut only_level = OnlyLevel::new();
    only_level.respawn(dma2d);
    rng::init_rng();

    loop {
        if only_level.process_frame(level, input, dma2d) {
            if let Some(next) = level.next() {
                level = next;
            } else {
                break;
            }
        }
        draw_and_wait();
    }

    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );

    0
}

struct OnlyLevel {
    player: Player,
    button_pressed: bool,
}

impl OnlyLevel {
    pub fn new() -> Self {
        OnlyLevel {
            player: Player::new(),
            button_pressed: false,
        }
    }

    pub fn process_frame(
        &mut self,
        level: &dyn LevelBehavior,
        input: &mut Inputs,
        dma2d: &mut Dma2d,
    ) -> bool {
        let old_hit_box: BoundingBox = self.player.hit_box.clone();
        self.player.erase_image(dma2d);
        self.player.velocity.x = level.calculate_player_vx(input, &self.player);
        self.player.velocity.y = level.calculate_player_vy(input, &self.player);
        self.player.hit_box.translate(&self.player.velocity);
        self.player.on_ground = false;
        
        environment::draw_button(dma2d, self.button_pressed);

        for wall in environment::WALL_HIT_BOXES.iter() {
            if let Some(collision_location) = self
                .player
                .hit_box
                .collides_with_interpolate(&old_hit_box, wall)
            {
                self.player
                    .push_out_of(&old_hit_box, collision_location, wall);
            }

            if wall.top_left.x < self.player.hit_box.bottom_right.x
                && wall.bottom_right.x > self.player.hit_box.top_left.x
            {
                if self.player.hit_box.bottom_right.y == wall.top_left.y {
                    self.player.on_ground = true;
                }
            }

            if self.player.hit_box.collides_with(wall) {
                defmt::info!("old player: {}", old_hit_box);
                defmt::info!("curr player: {}", self.player.hit_box);
                defmt::info!("wall: {}", wall);
                panic!();
            }
        }

        self.player.frames_in_air = if self.player.on_ground {
            0
        } else {
            self.player.frames_in_air + 1
        };

        if !self.button_pressed && self.player.hit_box.bottom_right.x > environment::GATE_HIT_BOX.top_left.x {
            if let Some(collision_location) = self
                .player
                .hit_box
                .collides_with_interpolate(&old_hit_box, &environment::GATE_HIT_BOX)
            {
                self.player.push_out_of(
                    &old_hit_box,
                    collision_location,
                    &environment::GATE_HIT_BOX,
                );
            }
        }

        for spike in environment::SPIKE_HIT_BOXES.iter() {
            if self.player.hit_box.collides_with(spike) {
                self.respawn(dma2d);
                return false;
            }
        }

        if !self.button_pressed
            && self
                .player
                .hit_box
                .collides_with(&environment::BUTTON_HIT_BOX)
        {
            environment::hide_gate(dma2d);
            self.button_pressed = true;
        }

        self.player.draw_image(dma2d);
        environment::draw_pipes(dma2d);

        environment::FINISH_PIPE_HIT_BOX.surrounds(&self.player.hit_box)
    }

    fn respawn(&mut self, dma2d: &mut Dma2d) {
        environment::draw_environment(dma2d);
        self.player
            .hit_box
            .translate_to(&environment::SPAWN_TOP_LEFT);
        self.button_pressed = false;
    }
}

pub(super) struct Player {
    pub(super) hit_box: BoundingBox,
    pub(super) velocity: Velocity,
    pub(super) on_ground: bool,
    pub(super) frames_in_air: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hit_box: BoundingBox::new(
                Position::new(1, 1),
                Position::new(
                    OnlyOneLevelPlayerImage::WIDTH as i32,
                    OnlyOneLevelPlayerImage::HEIGHT as i32,
                ),
            ),
            velocity: Velocity::new(0, 0),
            on_ground: false,
            frames_in_air: 0,
        }
    }

    pub fn push_out_of(
        &mut self,
        &old_hit_box: &BoundingBox,
        mut collision_location: BoundingBox,
        other_hit_box: &BoundingBox,
    ) {
        if self.velocity.x > 0 && other_hit_box.top_left.x >= old_hit_box.bottom_right.x {
            collision_location.translate(&Position::new(
                other_hit_box.top_left.x - collision_location.bottom_right.x,
                self.hit_box.top_left.y - collision_location.top_left.y,
            ));
            self.velocity.x = 0;
        } else if self.velocity.x < 0 && other_hit_box.bottom_right.x <= old_hit_box.top_left.x {
            collision_location.translate(&Position::new(
                other_hit_box.bottom_right.x - collision_location.top_left.x,
                self.hit_box.top_left.y - collision_location.top_left.y,
            ));
            self.velocity.x = 0;
        } else if self.velocity.y > 0 {
            collision_location.translate(&Position::new(
                0,
                other_hit_box.top_left.y - collision_location.bottom_right.y,
            ));
            self.velocity.y = 0;
        } else if self.velocity.y < 0 {
            collision_location.translate(&Position::new(
                0,
                other_hit_box.bottom_right.y - collision_location.top_left.y,
            ));
            self.velocity.y = 0;
        }

        self.hit_box = collision_location;
    }

    pub fn erase_image(&self, dma2d: &mut Dma2d) {
        dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerBackgroundImage.data_address(),
            core::cmp::max(0, self.hit_box.top_left.x) as u32,
            core::cmp::max(0, self.hit_box.top_left.y) as u32,
            OnlyOneLevelPlayerBackgroundImage::WIDTH,
            OnlyOneLevelPlayerBackgroundImage::HEIGHT,
        );
    }

    pub fn draw_image(&self, dma2d: &mut Dma2d) {
        dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerImage.data_address(),
            core::cmp::max(0, self.hit_box.top_left.x) as u32,
            core::cmp::max(0, self.hit_box.top_left.y) as u32,
            OnlyOneLevelPlayerImage::WIDTH,
            OnlyOneLevelPlayerImage::HEIGHT,
        );
    }
}
