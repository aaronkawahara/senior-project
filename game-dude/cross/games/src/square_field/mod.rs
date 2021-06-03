mod color_schemes;
mod player;
mod zones;

use crate::collisions::{BoundingBox, Collideable};
use crate::common::{DiscreteSelection, MovingObject, Position, Velocity};
use crate::images::SimpleImage;
use crate::images::{PlayerImage, SquareImage};
use crate::rng;
use crate::square_field::zones::EndZone;

use player::Player;
use board::input::Inputs;
use stm32l4p5_hal::dma2d::Dma2d;
use zones::Zones;

use self::color_schemes::ColorSchemes;
use self::zones::TransitionZone;

const BACKGROUND_COLOR: u32 = 0xff_ff_ff_ff;
const QUARTER_WIDTH: u16 = 120;

pub fn play(input: &mut Inputs, dma2d: &Dma2d, wait_for_vsync: fn() -> ()) -> u32 {
    let mut square_field = SquareField::new(dma2d);
    let mut game_over = false;
    rng::init();

    while !game_over {
        game_over = square_field.process_frame(input, dma2d);
        wait_for_vsync();
    }

    dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);
    square_field.score
}

pub(super) type Square = MovingObject<SquareImage>;
pub(super) type Field = [Square; SquareField::TOTAL_SQUARES as usize];

pub(super) struct SquareField<'a> {
    squares: Field,
    player: Player<'a>,
    score: u32,
    zone: Zones,
    color_scheme: ColorSchemes,
}

impl<'a> SquareField<'a> {
    pub(super) const X_MIN: i32 = -lcd::SCREEN_WIDTH_I32;
    pub(super) const X_MAX: i32 = lcd::SCREEN_WIDTH_I32 * 2;
    pub(super) const TOTAL_SQUARES: u16 = Self::ROWS * Self::SQUARES_PER_ROW;
    pub(super) const SQUARES_PER_ROW: u16 = 4;
    pub(super) const ROWS: u16 = lcd::SCREEN_HEIGHT_U16 / Self::ROW_SPACE + 2; // 1 for rounding up and 1 for smoothness
    pub(super) const ROW_SPACE: u16 = 2 * SquareImage::HEIGHT;

    pub fn new(dma2d: &'a Dma2d) -> Self {
        let square_hit_box = BoundingBox::new(
            Position::new(0, 0),
            Position::new(i32::from(SquareImage::WIDTH), i32::from(SquareImage::HEIGHT)),
        );

        let mut squares: Field = [Square::new(square_hit_box, Velocity::default(), SquareImage);
            Self::TOTAL_SQUARES as usize];

        let zone: Zones = Zones::init_start_zone(&mut squares);

        SquareField {
            squares,
            player: Player::new(dma2d),
            score: 0,
            zone,
            color_scheme: ColorSchemes::default(),
        }
    }

    pub fn process_frame(&mut self, input: &mut Inputs, dma2d: &Dma2d) -> bool {
        self.color_scheme.fill_background(dma2d);

        let mut game_over = false;

        let vx: i32 = match (input.left_pressed(), input.right_pressed()) {
            (false, false) | (true, true) => 0,
            (false, true) => -Player::MOVEMENT_SPEED,
            (true, false) => Player::MOVEMENT_SPEED,
        };

        let vy: i32 = self.zone.speed();

        self.player.draw(self.color_scheme);

        for square in &mut self.squares {
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

        if self.zone.passed_zone(self.score) {
            let zone = core::mem::take(&mut self.zone);

            if let Some(new_zone) = zone.next_zone(&mut self.squares) {
                self.zone = new_zone;

                if self.score > 2 * TransitionZone::TRANSITION_LENGTH 
                    && matches!(self.zone, Zones::Transition(_))
                {
                    self.color_scheme = self.color_scheme.next();
                }
            } else {
                game_over = true;
            }
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
                Self::X_MAX - i32::from(SquareImage::WIDTH),
                square.hit_box.top_left.y,
            ));
        }
    }
}
