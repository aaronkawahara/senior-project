use core::u32;

use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, PrimitiveArt, Velocity};
use crate::graphics::{egrectangle, egtriangle, prelude::*, Primitive, primitive_style, PrimitiveStyle, Rectangle, Styled, Triangle};
use crate::input::{DirectionalInput, DPad};
use lcd::{self, handle_draw, Lcd, RGB8};
use rand::{Rng, SeedableRng, rngs::SmallRng};

#[derive(Clone, Copy)]
struct Cube(MovingObject<Rectangle>);

impl Cube {
    const SIDE_LENGTH: i32 = 10;

    pub fn new(top_left: Position, velocity: Velocity) -> Cube {
        let bottom_right = Position::new(
            top_left.x + Self::SIDE_LENGTH, 
            top_left.y + Self::SIDE_LENGTH
        );

        let art: PrimitiveArt<Rectangle> = egrectangle!(
            top_left = (top_left.x, top_left.y),
            bottom_right = (bottom_right.x, bottom_right.y),
            style = primitive_style!(
                stroke_color = RGB8::new(RGB8::BLACK),
                fill_color = RGB8::new(RGB8::BLUE),
                stroke_width = 1
            )
        );

        Cube(
            MovingObject {
                hit_box: BoundingBox::new(Position::new(0, 0), bottom_right),
                velocity,
                art,
            }
        )
    }
}

struct Player(MovingObject<Triangle>);

impl Player {
    const PLAYER_WIDTH: i32 = 10;
    const PLAYER_HEIGHT: i32 = 10;

    pub fn new() -> Self {
        let x_mid = (lcd::SCREEN_WIDTH / 2) as i32;
        let x_min = x_mid - (Self::PLAYER_WIDTH / 2);
        let x_max = x_min + Self::PLAYER_WIDTH;
        let y_min = lcd::SCREEN_HEIGHT as i32;
        let y_max = y_min - Self::PLAYER_HEIGHT;

        let art: PrimitiveArt<Triangle> = egtriangle!(
            points = [
                (x_min, y_min), 
                (x_max, y_min),
                (x_mid, y_max)
            ],
            style = primitive_style!(
                stroke_color = RGB8::new(RGB8::BLACK),
                fill_color = RGB8::new(RGB8::BLUE),
                stroke_width = 1
            )
        );

        Player(
            MovingObject {
                hit_box: BoundingBox::new(Position::new(x_min, y_max), Position::new(x_max, y_min)),
                velocity: Velocity::new(0, 0),
                art
            }
        )
    }
}

const TOTAL_CUBES: u32 = 100;
const CUBES_PER_ROW: u32 = 10;
const ROWS: u32 = 10;
const ROW_SPACE: u32 = 20;
const ROW_WIDTH: u16 = 100;
const LEFT_BOUND: i32 = -100;
const RIGHT_BOUND: i32 = 580;
const CUBE_SPEED: i32 = 10;
const MOVEMENT_SPEED: i32 = 10;

struct CubeField {
    cubes: [Cube; TOTAL_CUBES as usize],
    player: Player,
}

impl CubeField {
    const RNG_SEED: [u8; 16] = [
        57, 15, 4, 218, 230, 117, 34, 242, 173, 21, 102, 234, 23, 225, 59, 137,
        // 180, 233, 32, 108, 41, 189, 248, 144, 83, 48, 250, 211, 129, 61, 22, 137
    ];

    pub fn new() -> Self {
        CubeField {
            cubes: [Cube::new(Position::new(0, 0), Velocity::new(0, 0)); TOTAL_CUBES as usize],
            player: Player::new()
        }
    }

    pub fn init(&mut self) {
        for row in 0..ROWS {
            let y = -((ROW_SPACE * row + 100) as i32);
            let delta = Position::new(0, y);

            for cube in 0..CUBES_PER_ROW {
                let i = (row * CUBES_PER_ROW + cube) as usize;
                self.cubes[i].0.translate(&delta);
                self.cubes[i].0.set_velocity(Velocity::new(0, -CUBE_SPEED));
            }

            self.distribute_row(row);
        }
    }

    pub fn player_collided_with_cube(&mut self) -> bool {
        let mut collision_detected = false;

        for cube in self.cubes.iter_mut() {
            cube.0.update_position();

            if cube.0.hit_box.top_left().x > RIGHT_BOUND {
                let y = cube.0.hit_box.top_left().y;
                cube.0.move_to_origin();
                cube.0.translate(&Position::new(LEFT_BOUND, y));
            } else if cube.0.hit_box.top_left().x < LEFT_BOUND {
                let y = cube.0.hit_box.top_left().y;
                cube.0.move_to_origin();
                cube.0.translate(&Position::new(RIGHT_BOUND - Cube::SIDE_LENGTH, y));
            }

            if cube.0.hit_box.collides_with(&self.player.0.hit_box) {
                collision_detected = true;
                break;
            }
        }

        collision_detected
    }

    pub fn process_input(&mut self, dpad: &DPad) {
        let vx = match (dpad.left_pressed(), dpad.right_pressed()) {
            (false, false) => 0,
            (false, true) => MOVEMENT_SPEED,
            (true, false) => -MOVEMENT_SPEED,
            (true, true) => 0,
        };

        for cube in self.cubes.iter_mut() {
            cube.0.set_velocity(Velocity::new(vx, 0));
        }
    }

    pub fn render(&self, lcd: &mut Lcd) {
        for cube in self.cubes.iter() {
            handle_draw(cube.0.art.draw(lcd));
        }

        handle_draw(self.player.0.art.draw(lcd));
    }

    fn distribute_row(&mut self, row_number: u32) {
        assert!(row_number < ROWS);

        let mut rng = SmallRng::from_seed(Self::RNG_SEED);
        let start_index: usize = (row_number * CUBES_PER_ROW) as usize;
        let end_index: usize = start_index + CUBES_PER_ROW as usize;

        for i in start_index..end_index {
            let delta = Position::new(rng.gen_range(LEFT_BOUND..RIGHT_BOUND), 0);
            self.cubes[i].0.translate(&delta);
        }
    }
}

pub fn play(lcd: &mut Lcd, dpad: &DPad, draw_and_wait: fn () -> ()) {
    let mut cube_field = CubeField::new();
    cube_field.init();

    let mut game_over = false;

    while !game_over {
        cube_field.process_input(dpad);
        game_over = cube_field.player_collided_with_cube();
        cube_field.render(lcd);
        draw_and_wait();
    }

    // end
    // tear down and return
}