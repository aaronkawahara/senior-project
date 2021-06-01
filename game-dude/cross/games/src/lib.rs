#![no_std]
#![feature(variant_count)]
#![feature(const_fn_trait_bound)]

mod collisions;
mod common;
mod high_scores;
mod images;
mod rng;
mod square_field;
mod only_one_level;
mod ui;

use crate::ui::{game_over, play_menu};

use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;

pub fn start_game_machine(mut input: Inputs, mut dma2d: Dma2d, wait_for_vsync: fn() -> ()) -> ! {
    let mut state: States = States::PlayMenu;

    loop {
        state = match state {
            States::PlayMenu => States::Play(play_menu::get_game_selection(
                &mut input,
                &dma2d,
                wait_for_vsync,
            )),
            States::Play(Games::SquareField) => {
                let score: u32 = square_field::play(&mut input, &dma2d, wait_for_vsync);
                States::GameOver {
                    game: Games::SquareField,
                    score,
                }
            }
            States::Play(Games::OnlyOneLevel) => {
                let score: u32 = only_one_level::play(&mut input, &dma2d, wait_for_vsync);
                States::GameOver {
                    game: Games::OnlyOneLevel,
                    score,
                }
            }
            States::GameOver { game, score } => {
                game_over::handle_game_over(&mut input, &dma2d, wait_for_vsync, game, score)
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
    OnlyOneLevel,
}
