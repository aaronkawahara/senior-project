use crate::common::Velocity;

use super::only_one_level::Player;

use board::input::Inputs;

pub(super) trait LevelBehavior {
    fn calculate_player_vx(&self, input: &mut Inputs, player: &Player) -> i32 {
        const MOVEMENT_SPEED: i32 = 3;

        match (input.left_pressed(), input.right_pressed()) {
            (true, false) => -MOVEMENT_SPEED,
            (false, true) => MOVEMENT_SPEED,
            _ => 0,
        }
    }

    fn calculate_player_vy(&self, input: &mut Inputs, player: &Player) -> i32 {
        const GRAVITY: i32 = 8;
        const FRAMES_TO_APEX: i32 = 30;
        const MAX_FALLING_VELOCITY: i32 = 8;
        const JUMP_SPEED: i32 = -8;

        if !player.on_ground {
            core::cmp::min(
                player.velocity.y + (GRAVITY * player.frames_in_air) / FRAMES_TO_APEX,
                MAX_FALLING_VELOCITY,
            )
        } else if input.up_pressed() {
            JUMP_SPEED
        } else {
            0
        }
    }

    fn next(&self) -> Option<&dyn LevelBehavior>;
}

pub(super) struct LevelOne;
// normal input
// normal init
impl LevelBehavior for LevelOne {
    fn next(&self) -> Option<&dyn LevelBehavior> {
        None
    }
}

pub(super) struct LevelTwo;
// controls inverted

pub(super) struct LevelThree;
// wall starts hidden
// button press shows wall

pub(super) struct LevelFour;
// lower gravity -> jump higher
// max falling velocity very slow

pub(super) struct LevelFive;
// veritcal wall touching bounces back with 70-90% velocity

pub(super) struct LevelSix;
// no jumping from input
// spikes bounce vertically on contact

pub(super) struct LevelSeven;
// stripes throughout level
// landing on the wrong stripe color kills you

pub(super) struct LevelEight;
// heavy constant head wind (right to left)
// walking barely fast
// jumping decently fast

pub(super) struct LevelNine;
// no walking backwards (left)

pub(super) struct LevelTen;
// no jumping
// gate starts hidden
// should be able to "walk" accross

pub(super) struct LevelEleven;
// gate starts hidden
// must alternate left right inputs to progress along predetermined path to finish pipe

pub(super) struct LevelTwelve;
// only one jump allowed
// must be able to make jump to button

pub(super) struct LevelThirteen;
// must hit button five times to open gate

pub(super) struct LevelFourteen;
// normal everything

pub(super) struct LevelFifteen;
// ground blocks fall from under after landing
// button does not open gate
// must go from above to get to finish

pub(super) struct LevelSixteen;
// game logic updates 10 times as slow

pub(super) struct LevelSeventeen;
// everything invisible except for pipes

pub(super) struct LevelEighteen;
// stripes throughout level
// one color makes you float up indefitely
// other color pulls down like normal

pub(super) struct LevelNineteen;
// invisible wall preceeding "boot" up to the to most platform

pub(super) struct LevelTwenty;
// gate goes down periodically and closes back up

pub(super) struct LevelTwentyOne;
// gate does not have hit box
// button does not lower gate

pub(super) struct LevelTwentyTwo;
// gate closes after set time

// ideas:
// die a certain number of times to hide gate
