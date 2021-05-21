#![no_std]

mod collisions;
mod common;
mod images;
mod rng;
mod square_field;
mod ui;

use crate::ui::game_over;

use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

pub fn start_game_machine(mut input: Inputs, mut dma2d: Dma2d, draw_and_wait: fn() -> ()) -> ! {
    let mut state: States = States::Play(Games::CubeField);

    loop {
        state = match state {
            States::Play(Games::CubeField) => {
                let score: u32 = square_field::play(&mut input, &mut dma2d, draw_and_wait);
                States::GameOver {
                    game: Games::CubeField,
                    score,
                }
            }
            States::GameOver { game, score } => {
                game_over::handle_game_over(&mut input, &mut dma2d, draw_and_wait, game, score)
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
