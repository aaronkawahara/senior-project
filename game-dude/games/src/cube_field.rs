use core::u32;

use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, Velocity};
use crate::graphics::{egrectangle, egtriangle, prelude::*, Primitive, primitive_style, PrimitiveStyle, Rectangle, RGB8, Styled};
use crate::input::DPad;
use lcd;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use crate::rng;

#[derive(Clone, Copy)]
struct Cube {
    hit_box: BoundingBox,
    velocity: Velocity,
    art: CubeArt,
}

impl Cube {
    const SIDE_LENGTH: i32 = 10;

    pub fn new(top_left: Position, velocity: Velocity) -> Cube {
        let bottom_right = Position::new(
            top_left.x + Self::SIDE_LENGTH, 
            top_left.y + Self::SIDE_LENGTH
        );

        let art: CubeArt = egrectangle!(
            top_left = (top_left.x, top_left.y),
            bottom_right = (bottom_right.x, bottom_right.y),
            style = primitive_style!(
                stroke_color = RGB8::new(RGB8::BLACK),
                fill_color = RGB8::new(RGB8::BLUE),
                stroke_width = 1
            )
        );

        Cube {
            hit_box: BoundingBox::new(top_left, bottom_right),
            velocity,
            art,
        }
    }

    pub fn move_to_origin(&mut self) {
        let negated_x = -self.hit_box.top_left().x;
        let negated_y = -self.hit_box.top_left().y;
        let negated_posn = Position::new(negated_x, negated_y);

        self.hit_box.translate(&negated_posn);
        self.art.translate_mut(Point::from(&negated_posn));
    }

    pub fn translate(&mut self, delta: &Position) {
        self.hit_box.translate(delta);
        self.art.translate_mut(Point::from(delta));
    }

    pub fn update_position(&mut self) {
        self.hit_box.translate(&self.velocity);
        self.art.translate_mut(Point::from(&self.velocity));
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}

// impl MovingObject for Cube {}

type CubeArt = Styled<Rectangle, PrimitiveStyle<RGB8>>;

const TOTAL_CUBES: u32 = 100;
const CUBES_PER_ROW: u32 = 10;
const ROWS: u32 = 10;
const ROW_SPACE: u32 = 20;
const ROW_WIDTH: u16 = 100;
const LEFT_BOUND: i32 = -100;
const RIGHT_BOUND: i32 = 580;
const CUBE_SPEED: i32 = 10;

struct CubeField {
    cubes: [Cube; TOTAL_CUBES as usize],
    player: Player,
}

impl CubeField {
    const PLAYER_WIDTH: i32 = 10;
    const RNG_SEED: [u8; 16] = [
        57, 15, 4, 218, 230, 117, 34, 242, 173, 21, 102, 234, 23, 225, 59, 137,
        // 180, 233, 32, 108, 41, 189, 248, 144, 83, 48, 250, 211, 129, 61, 22, 137
    ];

    pub fn new() -> Self {
        let x_mid = (lcd::SCREEN_WIDTH / 2) as i32;
        let player_art = egtriangle!(
            points = [(x_mid - (Self::PLAYER_WIDTH / 2), lcd::SCREEN_HEIGHT), ]
        )

        CubeField {
            cubes: [Cube::new(Position::new(0, 0), Velocity::new(0, 0)); TOTAL_CUBES as usize],
            player: Player {

            }
        }
    }

    pub fn init(&mut self) {
        for row in 0..ROWS {
            let y = -((ROW_SPACE * row + 100) as i32);
            let delta = Position::new(0, y);

            for cube in 0..CUBES_PER_ROW {
                let i = (row * CUBES_PER_ROW + cube) as usize;
                self.cubes[i].translate(&delta);
                self.cubes[i].set_velocity(Velocity::new(0, -CUBE_SPEED));
            }

            self.distribute_row(row);
        }
    }


    // todo maybe take input into this func for updating cube velocity before updating position
    pub fn step(&mut self)  {
        // move each cube based on velocity
        // if /* cube is off screen */ {
        //     // move row back and re-randomize
        // }
        for cube in self.cubes.iter_mut() {
            cube.update_position();

            if cube.hit_box.collides_with(other)
        }
    }

    fn distribute_row(&mut self, row_number: u32) {
        assert!(row_number < ROWS);

        let mut rng = SmallRng::from_seed(Self::RNG_SEED);
        let start_index: usize = (row_number * CUBES_PER_ROW) as usize;
        let end_index: usize = start_index + CUBES_PER_ROW as usize;

        for i in start_index..end_index {
            let delta = Position::new(rng.gen_range(LEFT_BOUND..RIGHT_BOUND), 0);
            self.cubes[i].translate(&delta);
        }
    }

    fn 
}

struct Player(Cube);

pub fn play(dpad: DPad) {
    let mut cube_field = CubeField::new();
    // cube_field.init();

    // play loop
    // get inputs
    // do physics/collisions
    // render screen
    // repeat until game ends

    // end
    // tear down and return
}