use core::f32;

use crate::common::{MovingObject, Position, Velocity};

use board::input::Inputs;

pub(super) trait Level {
    fn get_new_velocity(input: Inputs, vel_old: Velocity) -> Velocity;
}

pub(super) struct JumpData {
    frames_in_air: i32,
    gravity: f32,
}

impl JumpData {
    // -vi = 1/2 * a * dt
    const JUMP_VELOCITY: f32 = -1.0;
    const GRAVITY: f32 = 0.04;
    const DT: f32 = 50.0;

    pub fn new() -> Self {
        JumpData {
            frames_in_air: 0,
            gravity: Self::GRAVITY,
        }
    }

    pub fn fall(&mut self, old_vy: &i32) -> i32 {
        self.frames_in_air += 1;
        old_vy + (self.gravity / Self::DT).round() * self.frames_in_air
    }

    pub fn jump(&mut self) -> i32 {
        self.frames_in_air = 0;
        Self::JUMP_VELOCITY as i32
    }

    pub fn land(&mut self) -> i32 {
        self.frames_in_air = 0;
        self.gravity
    }
}

pub(super) struct LevelOne;
// normal controls

// impl Level for LevelOne {

// }

pub(super) struct LevelTwo;
// inverted left / right

pub(super) struct LevelThree;
// wall starts raised and pressing button closes it permanently

pub(super) struct LevelFour;
// lower gravity -> holding jump raises you indefinitely

pub(super) struct LevelFive;
// 