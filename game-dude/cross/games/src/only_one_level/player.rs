use stm32l4p5_hal::dma2d::Dma2d;

use crate::{
    collisions::BoundingBox,
    common::{Position, Velocity},
    images::{OnlyOneLevelPlayerBackgroundImage, OnlyOneLevelPlayerImage, SimpleImage},
};

use super::environment;

pub(super) struct Player<'a> {
    pub(super) hit_box: BoundingBox,
    pub(super) velocity: Velocity,
    pub(super) on_ground: bool,
    pub(super) frames_in_air: i32,
    physics: PlayerPhysics,
    dma2d: &'a Dma2d,
}

impl<'a> Player<'a> {
    pub fn new(dma2d: &'a Dma2d) -> Self {
        Player {
            hit_box: BoundingBox::new(
                Position::new(1, 1),
                Position::new(
                    i32::from(OnlyOneLevelPlayerImage::WIDTH),
                    i32::from(OnlyOneLevelPlayerImage::HEIGHT),
                ),
            ),
            velocity: Velocity::new(0, 0),
            on_ground: false,
            frames_in_air: 0,
            physics: PlayerPhysics::default(),
            dma2d,
        }
    }

    pub fn respawn(&mut self) {
        self.hit_box.translate_to(&environment::SPAWN_TOP_LEFT);
    }

    pub fn change_physics(&mut self, new_physics: PlayerPhysics) {
        self.physics = new_physics;
    }

    pub fn physics(&self) -> &PlayerPhysics {
        &self.physics
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
            // self.velocity.y = -(self.velocity.y * self.physics.bounce_factor_tenths) / 10;
            // let distance_to_ground = other_hit_box.top_left.y - old_hit_box.bottom_right.y;
            // let bounce_height = distance_to_ground + self.velocity.y;
            // collision_location.translate(&Position::new(
            //     0,
            //     other_hit_box.top_left.y - collision_location.bottom_right.y + bounce_height,
            // ));
            collision_location.translate(&Position::new(
                0,
                other_hit_box.top_left.y - collision_location.bottom_right.y,
            ));
            self.velocity.y = 0;
        } else if self.velocity.y < 0 {
            // self.velocity.y = -(self.velocity.y * self.physics.bounce_factor_tenths) / 10;
            // let distance_to_ceiling = other_hit_box.bottom_right.y - old_hit_box.top_left.y;
            // let bounce_depth = distance_to_ceiling + self.velocity.y;
            // collision_location.translate(&Position::new(
            //     0,
            //     other_hit_box.bottom_right.y - collision_location.top_left.y + bounce_depth,
            // ));
            collision_location.translate(&Position::new(
                0,
                other_hit_box.bottom_right.y - collision_location.top_left.y,
            ));
            self.velocity.y = 0;
        }

        self.hit_box = collision_location;
    }

    pub fn calculate_fall_speed(&self) -> i32 {
        core::cmp::min(
            self.velocity.y
                + (self.physics.gravity * self.frames_in_air) / self.physics.frames_to_apex,
            self.physics.max_falling_velocity,
        )
    }

    pub fn jump_speed(&self) -> i32 {
        self.physics.jump_speed
    }

    pub fn erase_image(&self) {
        self.dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerBackgroundImage.data_address(),
            core::cmp::max(0, self.hit_box.top_left.x) as u32,
            core::cmp::max(0, self.hit_box.top_left.y) as u32,
            OnlyOneLevelPlayerBackgroundImage::WIDTH,
            OnlyOneLevelPlayerBackgroundImage::HEIGHT,
        );
    }

    pub fn draw_image(&self) {
        self.dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerImage.data_address(),
            core::cmp::max(0, self.hit_box.top_left.x) as u32,
            core::cmp::max(0, self.hit_box.top_left.y) as u32,
            OnlyOneLevelPlayerImage::WIDTH,
            OnlyOneLevelPlayerImage::HEIGHT,
        );
    }
}
pub(super) struct PlayerPhysics {
    pub(super) gravity: i32,
    pub(super) frames_to_apex: i32,
    pub(super) max_falling_velocity: i32,
    pub(super) jump_speed: i32,
    pub(super) ground_speed: i32,
    pub(super) air_speed: i32,
    pub(super) bounce_factor_tenths: i32,
}

impl PlayerPhysics {
    pub(super) const GRAVITY: i32 = 8;
    pub(super) const FRAMES_TO_APEX: i32 = 30;
    pub(super) const MAX_FALLING_VELOCITY: i32 = 6;
    pub(super) const JUMP_SPEED: i32 = -8;
    pub(super) const GROUND_SPEED: i32 = 2;
    pub(super) const AIR_SPEED: i32 = 5;
    pub(super) const BOUNCE_FACTOR_TENTHS: i32 = 0;
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        PlayerPhysics {
            gravity: Self::GRAVITY,
            frames_to_apex: Self::FRAMES_TO_APEX,
            max_falling_velocity: Self::MAX_FALLING_VELOCITY,
            jump_speed: Self::JUMP_SPEED,
            ground_speed: Self::GROUND_SPEED,
            air_speed: Self::AIR_SPEED,
            bounce_factor_tenths: Self::BOUNCE_FACTOR_TENTHS,
        }
    }
}
