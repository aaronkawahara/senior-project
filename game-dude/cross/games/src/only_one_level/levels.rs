use crate::common::{MovingObject, Position, Velocity};

use board::input::Inputs;

const GRAVITY: i32 = -5;

pub(super) trait Level {
    fn get_new_velocity(input: Inputs, vel_old: Velocity) -> Velocity;
}

pub(super) struct LevelOne;
// normal controls

pub(super) struct LevelTwo;
// inverted left / right

pub(super) struct LevelThree;
// wall starts raised and pressing button closes it permanently

pub(super) struct LevelFour;
// lower gravity -> holding jump raises you indefinitely

pub(super) struct LevelFive;
// 