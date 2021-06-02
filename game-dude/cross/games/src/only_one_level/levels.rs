use crate::collisions::Collideable;

use super::{
    environment::{self, Environment},
    player::{Player, PlayerPhysics},
};

use board::input::Inputs;

pub(super) trait Level {
    fn init_environment(&self, environment: &mut Environment) {
        environment.draw_walls_and_spikes();
        environment.release_button();
        environment.draw_button();
        environment.close_gate();
        environment.draw_gate();
    }

    fn init_player(&self, player: &mut Player) {
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

    fn calculate_player_vy(&mut self, input: &mut Inputs, player: &Player) -> i32 {
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

    fn button_conditions_met(&mut self, player: &Player, environment: &Environment) -> bool {
        !environment.button_pressed() && player.hit_box.collides_with(&environment::BUTTON_HIT_BOX)
    }

    fn handle_button_press(&mut self, environment: &mut Environment) {
        environment.press_button();
        environment.open_gate();
        environment.draw_gate();
    }

    fn handle_spike_collision(&mut self, player: &mut Player, environment: &mut Environment) {
        self.init_player(player);
        self.init_environment(environment);
    }
}

pub(super) struct Normal;
impl Level for Normal {}

pub(super) struct InvertedControls;
impl Level for InvertedControls {
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

    fn calculate_player_vy(&mut self, input: &mut Inputs, player: &Player) -> i32 {
        if !player.on_ground {
            player.calculate_fall_speed()
        } else if input.down_pressed() {
            player.jump_speed()
        } else {
            0
        }
    }
}

pub(super) struct OpenGate;
impl Level for OpenGate {
    fn init_environment(&self, environment: &mut Environment) {
        environment.draw_walls_and_spikes();
        environment.release_button();
        environment.draw_button();
        environment.open_gate();
        environment.draw_gate();
    }

    fn handle_button_press(&mut self, environment: &mut Environment) {
        environment.press_button();
        environment.close_gate();
        environment.draw_gate();
    }
}

pub(super) struct Floaty;
impl Level for Floaty {
    fn init_player(&self, player: &mut Player) {
        const GRAVITY: i32 = 3;
        const MAX_FALLING_VELOCITY: i32 = 1;

        let new_physics = PlayerPhysics {
            gravity: GRAVITY,
            max_falling_velocity: MAX_FALLING_VELOCITY,
            ..PlayerPhysics::default()
        };

        player.change_physics(new_physics);
        player.respawn();
    }
}

pub(super) struct BouncyWalls;
impl Level for BouncyWalls {
    fn init_player(&self, player: &mut Player) {
        const BOUNCE_FACTOR_TENTHS: i32 = 8;

        let new_physics = PlayerPhysics {
            bounce_factor_tenths: BOUNCE_FACTOR_TENTHS,
            ..PlayerPhysics::default()
        };

        player.change_physics(new_physics);
        player.respawn();
    }
}

pub(super) struct BouncySpikes;
impl Level for BouncySpikes {
    fn calculate_player_vy(&mut self, _input: &mut Inputs, player: &Player) -> i32 {
        if player.on_ground {
            0
        } else {
            player.calculate_fall_speed()
        }
    }

    fn handle_spike_collision(&mut self, player: &mut Player, _environment: &mut Environment) {
        const BOUNCE_SPEED: i32 = -16;
        player.velocity.x = 0;
        player.velocity.y = BOUNCE_SPEED;
    }
}

pub(super) struct DeadlyStripes;
// stripes throughout level
// landing on the wrong stripe color kills you

pub(super) struct HeadWind;
impl Level for HeadWind {
    fn calculate_player_vx(&self, input: &mut Inputs, player: &Player) -> i32 {
        const HEAD_WIND: i32 = PlayerPhysics::GROUND_SPEED - 1;
        let player_physics = player.physics();

        (match (input.left_pressed(), input.right_pressed()) {
            (true, false) if player.on_ground => -player_physics.ground_speed,
            (false, true) if player.on_ground => player_physics.ground_speed,
            (true, false) if !player.on_ground => -player_physics.air_speed,
            (false, true) if !player.on_ground => player_physics.air_speed,
            _ => 0,
        }) - HEAD_WIND
    }
}

pub(super) struct NoRegrets;
impl Level for NoRegrets {
    fn calculate_player_vx(&self, input: &mut Inputs, player: &Player) -> i32 {
        let player_physics = player.physics();

        match (input.right_pressed(), player.on_ground) {
            (true, true) => player_physics.ground_speed,
            (true, false) => player_physics.air_speed,
            _ => 0,
        }
    }
}

pub(super) struct NoHops;
impl Level for NoHops {
    fn init_environment(&self, environment: &mut Environment) {
        environment.draw_walls_and_spikes();
        environment.release_button();
        environment.draw_button();
        environment.open_gate();
        environment.draw_gate();
    }

    fn calculate_player_vy(&mut self, _input: &mut Inputs, player: &Player) -> i32 {
        if player.on_ground {
            0
        } else {
            player.calculate_fall_speed()
        }
    }
}

pub(super) struct LevelEleven;
// gate starts hidden
// must alternate left right inputs to progress along predetermined path to finish pipe

#[derive(Default)]
pub(super) struct OneShot {
    used_jump: bool,
}
impl Level for OneShot {
    fn calculate_player_vy(&mut self, input: &mut Inputs, player: &Player) -> i32 {
        let player_physics = player.physics();

        if !self.used_jump && player.on_ground && input.up_pressed() {
            self.used_jump = true;
            player_physics.jump_speed
        } else if player.on_ground {
            0
        } else {
            player.calculate_fall_speed()
        }
    }

    fn handle_spike_collision(&mut self, player: &mut Player, environment: &mut Environment) {
        self.used_jump = false;
        self.init_player(player);
        self.init_environment(environment);
    }
}

#[derive(Default)]
pub(super) struct TryAgain {
    previous_frame_was_on_button: bool,
    button_hits: usize,
}
impl Level for TryAgain {
    fn button_conditions_met(&mut self, player: &Player, environment: &Environment) -> bool {
        let player_on_button: bool = !environment.button_pressed() && player.hit_box.collides_with(&environment::BUTTON_HIT_BOX);
        let result: bool = !self.previous_frame_was_on_button && player_on_button;
        self.previous_frame_was_on_button = player_on_button;

        result
    }

    fn handle_button_press(&mut self, environment: &mut Environment) {
        const PRESSES_REQUIRED: usize = 5;
        environment.button_pressed();
        
        self.button_hits += 1;
        if self.button_hits >= PRESSES_REQUIRED {
            environment.open_gate();
            environment.draw_gate();
        }
    }

    fn handle_spike_collision(&mut self, player: &mut Player, environment: &mut Environment) {
        self.init_player(player);
        self.init_environment(environment);

        self.previous_frame_was_on_button = false;
        self.button_hits = 0;
    }
}

pub(super) struct LevelFourteen;
// normal everything

pub(super) struct LevelFifteen;
// ground blocks fall from under after landing
// button does not open gate
// must go from above to get to finish

pub(super) struct LevelSixteen;
// game logic updates 10 times as slow

pub(super) struct DoYouRemember;
impl Level for DoYouRemember {
    fn init_environment(&self, environment: &mut Environment) {
        environment.hide_walls();
        environment.hide_gate();
        environment.hide_button();
        environment.draw_walls_and_spikes();
        environment.draw_pipes();
    }
}

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

pub(super) struct Sacrifice;
impl Level for Sacrifice {
    fn button_conditions_met(&mut self, player: &Player, environment: &Environment) -> bool {
        false
    }

    fn handle_spike_collision(&mut self, player: &mut Player, environment: &mut Environment) {
        self.init_environment(environment);
        self.init_player(player);
        environment.press_button();
        environment.open_gate();
        environment.draw_gate();
    }
}