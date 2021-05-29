mod environment;
mod levels;
mod player;

use crate::collisions::{BoundingBox, Collideable};
use crate::rng;

use environment::Environment;
use levels::*;
use player::Player;

use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn play(input: &mut Inputs, dma2d: &Dma2d, draw_and_wait: fn() -> ()) -> u32 {
    rng::init();
    let mut level: usize = 1;

    // while level <= levels::LAST_LEVEL {
    //     level = match level {
    //         1 => { OnlyLevel::new(dma2d, LevelOne).play(input, draw_and_wait); 2 }
    //         2 => { OnlyLevel::new(dma2d, LevelTwo).play(input, draw_and_wait); 3 }
    //         _ => panic!("current level exceeds intended limit")
    //     };
    // }

    OnlyLevel::new(dma2d, BouncyWalls).play(input, draw_and_wait);

    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );

    0
}

struct OnlyLevel<'d, L: Level> {
    player: Player<'d>,
    environment: Environment<'d>,
    level: L,
}

impl<'d, L: Level> OnlyLevel<'d, L> {
    pub fn new(dma2d: &'d Dma2d, level: L) -> Self {
        let mut player = Player::new(dma2d);
        let mut environment = Environment::new(dma2d);
        level.init_environment(&mut environment);
        level.init_player(&mut player);

        OnlyLevel {
            player,
            environment,
            level,
        }
    }

    pub fn play(&mut self, input: &mut Inputs, draw_and_wait: fn() -> ()) {
        while !self.process_frame(input) {
            draw_and_wait();
        }
    }

    pub fn process_frame(&mut self, input: &mut Inputs) -> bool {
        self.environment.draw_button();
        let old_hit_box: BoundingBox = self.player.hit_box;
        self.player.erase_image();
        self.player.velocity.x = self.level.calculate_player_vx(input, &self.player);
        self.player.velocity.y = self.level.calculate_player_vy(input, &self.player);
        self.player.hit_box.translate(&self.player.velocity);
        self.player.on_ground = false;

        for wall in &environment::WALL_HIT_BOXES {
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
                && self.player.hit_box.bottom_right.y == wall.top_left.y
            {
                self.player.on_ground = true;
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

        if !self.environment.gate_opened()
            && self.player.hit_box.bottom_right.x > environment::GATE_HIT_BOX.top_left.x
        {
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

        for spike in &environment::SPIKE_HIT_BOXES {
            if self.player.hit_box.collides_with(spike) {
                self.level
                    .handle_spike_collision(&mut self.player, &mut self.environment);
            }
        }

        if self
            .level
            .button_conditions_met(&self.player, &self.environment)
        {
            self.level.handle_button_press(&mut self.environment);
        }

        self.player.draw_image();
        self.environment.draw_pipes();

        environment::FINISH_PIPE_HIT_BOX.surrounds(&self.player.hit_box)
    }
}
