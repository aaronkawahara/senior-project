use crate::collisions::BoundingBox;
use crate::common::{MovingObject, Position, Velocity};
use crate::graphics::{egrectangle, prelude::*, Primitive, primitive_style, PrimitiveStyle, Rectangle, RGB8, Styled};
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
}

impl MovingObject for Cube {
    fn update_position(&mut self) {
        self.hit_box.translate(&self.velocity);
        self.art.translate_mut(Point::from(&self.velocity));
    }
}

type CubeArt = Styled<Rectangle, PrimitiveStyle<RGB8>>;

const TOTAL_CUBES: u32 = 100;
const CUBES_PER_ROW: u32 = 10;
const ROWS: u32 = 10;
const ROW_SPACE: u32 = 20;
const ROW_WIDTH: u16 = 100;

struct CubeField {
    cubes: [Cube; TOTAL_CUBES as usize],
}

impl CubeField {
    const RNG_SEED: [u8; 32] = [
        57, 15, 4, 218, 230, 117, 34, 242, 173, 21, 102, 234, 23, 225, 59, 137,
        180, 233, 32, 108, 41, 189, 248, 144, 83, 48, 250, 211, 129, 61, 22, 137
    ];

    pub fn new() -> Self {
        CubeField {
            cubes: [Cube::new(Position::new(0, 0), Velocity::new(0, 0)); TOTAL_CUBES as usize]
        }
    }

    pub fn init(&mut self) {
        // modify y positions based on cubes per row
        // modify x position randomly
    }

    pub fn step(&mut self)  {
        // move each cube based on velocity
        // if /* cube is off screen */ {
        //     // move row back and re-randomize
        // }
    }

    fn distribute_row(&mut self, row_number: u32) {
        let mut seed: <rng::SimpleRng as SeedableRng>::Seed = rng::SimpleRngSeed(Self::RNG_SEED);
        let mut rng = SmallRng::from_seed(seed);
        // randomly modify x positions of cubes in this row
    }
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