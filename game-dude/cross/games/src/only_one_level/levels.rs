use crate::common::{MovingObject, Position, Velocity};

use board::input::Inputs;

pub(super) trait Level {
    fn get_new_velocity(input: Inputs, vel_old: Velocity) -> Velocity;
}

pub(super) struct JumpData {
    frames_in_air: i32,
    gravity: i32,
}

impl JumpData {
    // -vi = 1/2 * a * dt
    const JUMP_VELOCITY: i32 = -8;
    const GRAVITY: i32 = 8;
    const FRAMES_TO_APEX: i32 = 30;
    const MAX_FALLING_VELOCITY: i32 = 8;

    pub fn new() -> Self {
        JumpData {
            frames_in_air: 0,
            gravity: 0,
        }
    }

    pub fn fall(&mut self, old_vy: &i32) -> i32 {
        self.frames_in_air += 1;
        core::cmp::min(old_vy + (Self::GRAVITY * self.frames_in_air) / Self::FRAMES_TO_APEX, Self::MAX_FALLING_VELOCITY)
    }

    pub fn jump(&mut self) -> i32 {
        self.frames_in_air = 0;
        Self::JUMP_VELOCITY
    }

    pub fn landed(&mut self) {
        self.frames_in_air = 0;
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