#![no_std]

pub mod input;
pub mod square_field;
pub mod ui;

mod collisions;
mod common;
mod images;
mod rng;

use crate::ui::game_over;

use input::DPad;
use stm32l4p5_hal::dma2d::Dma2d;

pub fn start_game_machine(mut dpad: DPad, mut dma2d: Dma2d, draw_and_wait: fn() -> ()) -> ! {
    let mut state: States = States::Play(Games::CubeField);

    loop {
        state = match state {
            States::Play(Games::CubeField) => {
                let score: u32 = square_field::play(&mut dpad, &mut dma2d, draw_and_wait);
                States::GameOver {
                    game: Games::CubeField,
                    score,
                }
            }
            States::GameOver { game, score } => {
                game_over::handle_game_over(&mut dpad, &mut dma2d, draw_and_wait, game, score)
            }
            States::MainMenu => States::MainMenu,
        };
    }
}
pub(crate) enum States {
    GameOver { game: Games, score: u32 },
    MainMenu,
    Play(Games),
}

#[derive(Clone, Copy)]
pub(crate) enum Games {
    CubeField,
}
