#![no_std]
#![feature(variant_count)]

mod collisions;
mod common;
mod images;
mod rng;
mod square_field;
mod ui;

use crate::ui::{game_over, play_menu};

use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

pub fn start_game_machine(mut input: Inputs, mut dma2d: Dma2d, draw_and_wait: fn() -> ()) -> ! {
    let mut state: States = States::PlayMenu;

    loop {
        state = match state {
            States::PlayMenu => States::Play(play_menu::get_game_selection(
                &mut input,
                &mut dma2d,
                draw_and_wait,
            )),
            States::Play(Games::SquareField) => {
                let score: u32 = square_field::play(&mut input, &mut dma2d, draw_and_wait);
                States::GameOver {
                    game: Games::SquareField,
                    score,
                }
            }
            States::GameOver { game, score } => {
                game_over::handle_game_over(&mut input, &mut dma2d, draw_and_wait, game, score)
            }
        };
    }
}
pub(crate) enum States {
    GameOver { game: Games, score: u32 },
    PlayMenu,
    Play(Games),
}

#[derive(Clone, Copy)]
pub(crate) enum Games {
    SquareField,
}
