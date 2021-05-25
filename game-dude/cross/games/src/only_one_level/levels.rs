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
    const JUMP_VELOCITY: i32 = -1;
    const GRAVITY: i32 = 1;
    const JUMP_FRAMES: i32 = 100;

    pub fn new() -> Self {
        JumpData {
            frames_in_air: 0,
            gravity: Self::GRAVITY,
        }
    }

    pub fn fall(&mut self, old_vy: &i32) -> i32 {
        self.frames_in_air += 1;
        old_vy + (self.gravity * self.frames_in_air) / Self::JUMP_FRAMES
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