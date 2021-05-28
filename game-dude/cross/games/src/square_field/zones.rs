use crate::common::Position;
use crate::images::SimpleImage;
use crate::images::SquareImage;
use crate::rng;

use super::{Field, Square, SquareField};

pub(super) enum Zones {
    Empty(EmptyZone),
    Transition(TransitionZone),
    Random(RandomizedZone),
    End(EndZone),
}

impl Zones {
    pub(super) fn init_start_zone(squares: &mut Field) -> Zones {
        Zones::Empty(EmptyZone::default().setup(squares))
    }

    pub fn reposition_square(&mut self, square: &mut Square) {
        match self {
            Zones::Empty(_) => {}
            Zones::Transition(z) => z.reposition_square(square),
            Zones::Random(z) => z.reposition_square(square),
            Zones::End(_) => {}
        }
    }

    pub fn next_zone(self, score: u32, squares: &mut Field) -> Option<Zones> {
        match self {
            Zones::Empty(z) if z.passed_zone(score) => Some(z.next_zone(squares)),
            Zones::Transition(z) if z.passed_zone(score) => Some(z.next_zone(squares)),
            Zones::Random(z) if z.passed_zone(score) => Some(z.next_zone(squares)),
            Zones::End(_) => None,
            _zone => Some(_zone),
        }
    }

    pub fn speed(&self) -> i32 {
        match self {
            Zones::Empty(z) => z.square_speed(),
            Zones::Transition(z) => z.square_speed(),
            Zones::Random(z) => z.square_speed(),
            Zones::End(_) => 0,
        }
    }
}

impl Default for Zones {
    fn default() -> Self {
        Zones::End(EndZone)
    }
}

pub(super) trait ZoneBehavior {
    fn square_speed(&self) -> i32;
    fn end(&self) -> u32;
    fn next_zone(self, squares: &mut Field) -> Zones;
    fn reposition_square(&mut self, square: &mut Square);
    fn setup(self, squares: &mut Field) -> Self;

    fn passed_zone(&self, score: u32) -> bool {
        score >= self.end()
    }
}

pub(super) struct EmptyZone {
    end: u32,
    level_data: LevelData,
    transition_next: bool,
}

impl EmptyZone {
    const ZONE_LENGTH: u32 = 2 * lcd::SCREEN_HEIGHT_U32;

    pub fn default() -> Self {
        EmptyZone {
            end: Self::ZONE_LENGTH,
            level_data: LevelData::default(),
            transition_next: true,
        }
    }

    fn new(level_data: LevelData, start: u32, transition_next: bool) -> Self {
        EmptyZone {
            end: start + Self::ZONE_LENGTH,
            level_data,
            transition_next,
        }
    }
}

impl ZoneBehavior for EmptyZone {
    fn square_speed(&self) -> i32 {
        self.level_data.square_speed
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn next_zone(self, squares: &mut Field) -> Zones {
        if self.transition_next {
            Zones::Transition(TransitionZone::new(self.level_data, self.end).setup(squares))
        } else {
            Zones::Random(
                RandomizedZone::new(self.level_data, self.end + lcd::SCREEN_HEIGHT_U32)
                    .setup(squares),
            )
        }
    }

    fn reposition_square(&mut self, _square: &mut Square) {}

    fn setup(self, squares: &mut Field) -> Self {
        for square in squares.iter_mut() {
            square
                .hit_box
                .translate_to(&Position::new(0, lcd::SCREEN_WIDTH_I32));
        }

        self
    }
}

pub(super) struct TransitionZone {
    curr_square: u16,
    end: u32,
    level_data: LevelData,
    rows_passed: i32,
}

impl TransitionZone {
    const BASE_SPEED: i32 = 5;
    const SPEED_INCREMENT: i32 = 2;
    const DX: i32 = SquareImage::WIDTH as i32;
    const LEFT_WALL_START: i32 = -(lcd::SCREEN_WIDTH_I32 / 2) + (SquareImage::WIDTH as i32) / 2;
    // const LEFT_WALL_END: i32 = lcd::SCREEN_WIDTH_I32 / 4 - (SquareImage::WIDTH as i32) / 2;
    const RIGHT_WALL_START: i32 = (lcd::SCREEN_WIDTH_I32 + lcd::SCREEN_WIDTH_I32 / 2)
        - (SquareImage::WIDTH as i32) / 2
        - SquareImage::WIDTH as i32;
    const RIGHT_WALL_END: i32 = (lcd::SCREEN_WIDTH_I32 * 3) / 4;
    const CORRAL_ROWS: i32 = (Self::RIGHT_WALL_START - Self::RIGHT_WALL_END) / Self::DX + 1;
    const RUNWAY_ROWS: i32 = SquareField::ROWS as i32;
    const TRANSITION_ROWS: i32 = Self::CORRAL_ROWS + Self::RUNWAY_ROWS;
    const TRANSITION_LENGTH: u32 = Self::TRANSITION_ROWS as u32 * SquareField::ROW_SPACE as u32;

    fn new(mut level_data: LevelData, zone_start: u32) -> Self {
        level_data.square_speed = Self::BASE_SPEED + (Self::SPEED_INCREMENT * level_data.level);

        TransitionZone {
            curr_square: 0,
            end: zone_start + Self::TRANSITION_LENGTH,
            level_data,
            rows_passed: 0,
        }
    }
}

impl ZoneBehavior for TransitionZone {
    fn reposition_square(&mut self, square: &mut Square) {
        let x: i32 = match self.curr_square {
            0 => {
                Self::DX * (Self::CORRAL_ROWS - self.rows_passed).clamp(0, SquareField::ROWS as i32)
            }
            1 => {
                -Self::DX
                    * (Self::CORRAL_ROWS - self.rows_passed).clamp(0, SquareField::ROWS as i32)
            }
            _ => 0,
        };

        let y: i32 = -((SquareField::ROW_SPACE * SquareField::ROWS) as i32);

        square.hit_box.translate(&Position::new(x, y));

        self.curr_square += 1;
        if self.curr_square == SquareField::SQUARES_PER_ROW {
            self.rows_passed += 1;
            self.curr_square = 0;
        }
    }

    fn next_zone(self, _squares: &mut Field) -> Zones {
        Zones::Empty(EmptyZone::new(self.level_data, self.end, false))
    }

    fn square_speed(&self) -> i32 {
        self.level_data.square_speed
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn setup(self, squares: &mut Field) -> Self {
        for row in 0..(SquareField::ROWS as i32) {
            let y: i32 = lcd::SCREEN_HEIGHT_I32
                - (SquareImage::HEIGHT as i32 + (SquareField::ROW_SPACE as i32) * row);

            for square in 0..SquareField::SQUARES_PER_ROW {
                let x: i32 = match square {
                    0 => Self::LEFT_WALL_START + row * Self::DX,
                    1 => Self::RIGHT_WALL_START - row * Self::DX,
                    _ => SquareField::X_MIN,
                };

                let i = (row as u16 * SquareField::SQUARES_PER_ROW + square) as usize;
                squares[i].hit_box.translate_to(&Position::new(x, y));
            }
        }

        self
    }
}

pub(super) struct RandomizedZone {
    end: u32,
    level_data: LevelData,
}

impl RandomizedZone {
    const BASE_ZONE_LENGTH: u32 = 2_000;
    const ZONE_INCREMENT: u32 = TransitionZone::SPEED_INCREMENT as u32 * 2000;

    fn new(level_data: LevelData, zone_start: u32) -> Self {
        RandomizedZone {
            end: zone_start
                + (Self::BASE_ZONE_LENGTH + Self::ZONE_INCREMENT * level_data.level as u32),
            level_data,
        }
    }
}

impl ZoneBehavior for RandomizedZone {
    fn reposition_square(&mut self, square: &mut Square) {
        let new_position = Position::new(
            rng::gen_range(SquareField::X_MIN..SquareField::X_MAX),
            square.hit_box.top_left.y - (SquareField::ROW_SPACE * SquareField::ROWS) as i32,
        );

        square.hit_box.translate_to(&new_position);
    }

    fn next_zone(mut self, _squares: &mut Field) -> Zones {
        self.level_data.level += 1;

        let min_space_needed = EmptyZone::ZONE_LENGTH
            + TransitionZone::TRANSITION_LENGTH
            + Self::BASE_ZONE_LENGTH
            + Self::ZONE_INCREMENT * self.level_data.level as u32;

        if self.end < u32::MAX - min_space_needed {
            Zones::Empty(EmptyZone::new(self.level_data, self.end, true))
        } else {
            Zones::End(EndZone)
        }
    }

    fn square_speed(&self) -> i32 {
        self.level_data.square_speed
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn setup(self, squares: &mut Field) -> Self {
        for row in 0..(SquareField::ROWS as i32) {
            let y: i32 = -(SquareImage::HEIGHT as i32 + (SquareField::ROW_SPACE as i32) * row);

            for square in 0..SquareField::SQUARES_PER_ROW {
                let x: i32 = rng::gen_range(SquareField::X_MIN..SquareField::X_MAX);

                let i = (row as u16 * SquareField::SQUARES_PER_ROW + square) as usize;
                squares[i].hit_box.translate_to(&Position::new(x, y));
            }
        }

        self
    }
}

pub(super) struct EndZone;

struct LevelData {
    square_speed: i32,
    level: i32,
}

impl LevelData {
    const START_SPEED: i32 = 5;

    pub fn default() -> Self {
        LevelData {
            square_speed: Self::START_SPEED,
            level: 0,
        }
    }
}
