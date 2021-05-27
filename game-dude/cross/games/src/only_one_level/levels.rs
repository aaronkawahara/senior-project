use crate::{collisions::Collideable, common::Velocity};

use super::{
    environment::{self, Environment},
    player::*,
};

use board::input::Inputs;

pub(super) const LAST_LEVEL: usize = 2;

pub(super) trait LevelBehavior {
    fn init(&self, player: &mut Player, environment: &mut Environment) {
        environment.release_button();
        environment.close_gate();
        environment.draw_walls_and_spikes();
        environment.draw_gate();
        environment.draw_button();
        player.change_physics(PlayerPhysics::default());
        player.respawn();
    }

    fn calculate_player_vx(&self, input: &mut Inputs, player: &Player) -> i32 {
        let player_physics = player.physics();

        match (input.left_pressed(), input.right_pressed()) {
            (true, false) if player.on_ground => -player_physics.ground_speed,
            (false, true) if player.on_ground => player_physics.ground_speed,
            (true, false) if !player.on_ground => -player_physics.air_speed,
            (false, true) if !player.on_ground => player_physics.air_speed,
            _ => 0,
        }
    }

    fn calculate_player_vy(&self, input: &mut Inputs, player: &Player) -> i32 {
        let player_physics = player.physics();

        if !player.on_ground {
            core::cmp::min(
                player.velocity.y
                    + (player_physics.gravity * player.frames_in_air)
                        / player_physics.frames_to_apex,
                player_physics.max_falling_velocity,
            )
        } else if input.up_pressed() {
            player_physics.jump_speed
        } else {
            0
        }
    }

    fn button_conditions_met(&self, player: &Player, environment: &Environment) -> bool {
        !environment.button_pressed() && player.hit_box.collides_with(&environment::BUTTON_HIT_BOX)
    }

    fn handle_button_press(&self, environment: &mut Environment) {
        environment.press_button();
        environment.open_gate();
        environment.draw_gate();
    }

    fn handle_spike_collision(&self, player: &mut Player, environment: &mut Environment) {
        self.init(player, environment);
    }
}

pub(super) struct LevelOne;
impl LevelBehavior for LevelOne {}

pub(super) struct LevelTwo;
impl LevelBehavior for LevelTwo {
    // controls inverted
    
    fn calculate_player_vx(&self, input: &mut Inputs, player: &Player) -> i32 {
        let player_physics = player.physics();

        match (input.left_pressed(), input.right_pressed()) {
            (true, false) if player.on_ground => player_physics.ground_speed,
            (false, true) if player.on_ground => -player_physics.ground_speed,
            (true, false) if !player.on_ground => player_physics.air_speed,
            (false, true) if !player.on_ground => -player_physics.air_speed,
            _ => 0,
        }
    }

    fn calculate_player_vy(&self, input: &mut Inputs, player: &Player) -> i32 {
        let player_physics = player.physics();

        if !player.on_ground {
            core::cmp::min(
                player.velocity.y
                    + (player_physics.gravity * player.frames_in_air)
                        / player_physics.frames_to_apex,
                player_physics.max_falling_velocity,
            )
        } else if input.down_pressed() {
            player_physics.jump_speed
        } else {
            0
        }
    }
}

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
