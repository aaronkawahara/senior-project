use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, Velocity};
use crate::images::SimpleImage;
use crate::images::{PlayerImage, SquareImage};
use crate::rng;

use super::zones::Zones;

use board::input::Inputs;
use lcd;
use stm32l4p5_hal::dma2d::Dma2d;

const BACKGROUND_COLOR: u32 = 0xff_ff_ff_ff;
const QUARTER_WIDTH: u16 = 120;

pub fn play(input: &mut Inputs, dma2d: &mut Dma2d, draw_and_wait: fn() -> ()) -> u32 {
    let mut square_field = SquareField::new();
    let mut game_over = false;
    rng::init_rng();

    while !game_over {
        game_over = square_field.process_frame(input, dma2d);
        draw_and_wait();
    }

    dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);
    square_field.score
}

pub(super) type Square = MovingObject<SquareImage>;
pub(super) type Player = MovingObject<PlayerImage>;
pub(super) type Field = [Square; SquareField::TOTAL_SQUARES as usize];

trait MovesOnInput {
    const MOVEMENT_SPEED: i32;
}

impl MovesOnInput for Player {
    const MOVEMENT_SPEED: i32 = 15;
}

pub(super) struct SquareField {
    squares: Field,
    player: Player,
    score: u32,
    zone: Zones,
}

impl SquareField {
    pub(super) const X_MIN: i32 = -lcd::SCREEN_WIDTH_I32;
    pub(super) const X_MAX: i32 = lcd::SCREEN_WIDTH_I32 * 2;
    pub(super) const TOTAL_SQUARES: u16 = Self::ROWS * Self::SQUARES_PER_ROW;
    pub(super) const SQUARES_PER_ROW: u16 = 4;
    pub(super) const ROWS: u16 = lcd::SCREEN_HEIGHT_U16 / Self::ROW_SPACE + 2; // 1 for rounding up and 1 for smoothness
    pub(super) const ROW_SPACE: u16 = 2 * SquareImage::HEIGHT;

    pub fn new() -> Self {
        let square_hit_box = BoundingBox::new(
            Position::new(0, 0),
            Position::new(SquareImage::WIDTH as i32, SquareImage::HEIGHT as i32),
        );

        let px: i32 = lcd::SCREEN_WIDTH_I32 / 2 - (PlayerImage::WIDTH / 2) as i32;
        let py: i32 = lcd::SCREEN_HEIGHT_I32 - 10 - PlayerImage::HEIGHT as i32;

        let player_hit_box = BoundingBox::new(
            Position::new(px, py),
            Position::new(
                px + PlayerImage::WIDTH as i32,
                py + PlayerImage::HEIGHT as i32,
            ),
        );

        let mut squares: Field = [Square::new(square_hit_box, Velocity::default(), SquareImage);
            Self::TOTAL_SQUARES as usize];

        let zone: Zones = Zones::init_start_zone(&mut squares);

        SquareField {
            squares,
            player: Player::new(player_hit_box, Velocity::default(), PlayerImage),
            score: 0,
            zone,
        }
    }

    pub fn process_frame(&mut self, input: &mut Inputs, dma2d: &mut Dma2d) -> bool {
        dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);

        let mut game_over = false;

        let vx: i32 = match (input.left_pressed(), input.right_pressed()) {
            (false, false) => 0,
            (false, true) => -Player::MOVEMENT_SPEED,
            (true, false) => Player::MOVEMENT_SPEED,
            (true, true) => 0,
        };

        let vy: i32 = self.zone.speed();

        dma2d.draw_rgb8_image(
            self.player.image.data_address(),
            self.player.hit_box.top_left.x as u32,
            self.player.hit_box.top_left.y as u32,
            PlayerImage::WIDTH,
            PlayerImage::HEIGHT,
        );

        for square in self.squares.iter_mut() {
            if square.hit_box.collides_with(&self.player.hit_box) {
                game_over = true;
                break;
            }

            square.set_velocity(Velocity::new(vx, vy));
            square.hit_box.translate(&square.velocity);

            if Self::object_on_screen(&square.hit_box) {
                // TODO refactor the cropping logic to common or collisionss
                let cropped_box = BoundingBox::new(
                    Position::new(
                        square.hit_box.top_left.x.clamp(0, lcd::SCREEN_WIDTH_I32),
                        square.hit_box.top_left.y.clamp(0, lcd::SCREEN_HEIGHT_I32),
                    ),
                    Position::new(
                        square
                            .hit_box
                            .bottom_right
                            .x
                            .clamp(0, lcd::SCREEN_WIDTH_I32),
                        square
                            .hit_box
                            .bottom_right
                            .y
                            .clamp(0, lcd::SCREEN_HEIGHT_I32),
                    ),
                );

                let cropped_offset = cropped_box.top_left - square.hit_box.top_left;

                let cropped_address: u32 = square
                    .image
                    .data_address_offset(cropped_offset.x as u16, cropped_offset.y as u16);

                dma2d.draw_rgb8_image(
                    cropped_address,
                    cropped_box.top_left.x as u32,
                    cropped_box.top_left.y as u32,
                    cropped_box.width() as u16,
                    cropped_box.height() as u16,
                );
            }

            if square.hit_box.top_left.y > lcd::SCREEN_HEIGHT_I32 {
                self.zone.reposition_square(square);
            }

            Self::wrap_square_if_out_of_bounds(square);
        }

        self.score += vy as u32;

        let zone = core::mem::take(&mut self.zone);
        if let Some(new_zone) = zone.next_zone(self.score, &mut self.squares) {
            self.zone = new_zone;
        } else {
            game_over = true;
        }

        game_over
    }

    fn object_on_screen(obj_hit_box: &BoundingBox) -> bool {
        let screen = BoundingBox::new(
            Position::new(0, 0),
            Position::new(lcd::SCREEN_WIDTH_I32, lcd::SCREEN_HEIGHT_I32),
        );

        screen.collides_with(obj_hit_box)
    }

    fn wrap_square_if_out_of_bounds(square: &mut Square) {
        if square.hit_box.top_left.x > Self::X_MAX {
            square
                .hit_box
                .translate_to(&Position::new(Self::X_MIN, square.hit_box.top_left.y));
        } else if square.hit_box.bottom_right.x < Self::X_MIN {
            square.hit_box.translate_to(&Position::new(
                Self::X_MAX - SquareImage::WIDTH as i32,
                square.hit_box.top_left.y,
            ));
        }
    }
}
