use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, PrimitiveArt, Velocity};
use crate::graphics::{egrectangle, egtriangle, prelude::*, Primitive, primitive_style, PrimitiveStyle, Rectangle, Styled, Triangle};
use crate::input::{DirectionalInput, DPad};
use lcd::{self, handle_draw, Lcd, RGB8};
use rand::{Rng, rngs::SmallRng};

pub fn play(lcd: &mut Lcd, dpad: &DPad, rng: &mut SmallRng, draw_and_wait: fn () -> ()) {
    let mut cube_field = CubeField::new();
    cube_field.init(rng);

    let mut game_over = false;

    while !game_over {
        game_over = cube_field.process_frame(lcd, dpad, rng);
        draw_and_wait();
    }
}

const TOTAL_CUBES: u16 = 12;
const CUBES_PER_ROW: u16 = 3;
const ROWS: u16 = 4;
const ROW_SPACE: u16 = 200;
const LEFT_BOUND: i32 = -50;
const RIGHT_BOUND: i32 = 430;
const TOP_BOUND: i32 = -50;
const BOTTOM_BOUND: i32 = 222;
const CUBE_SPEED: i32 = 5;
const MOVEMENT_SPEED: i32 = 10;

struct CubeField {
    cubes: [Cube; TOTAL_CUBES as usize],
    player: Player,
}

impl CubeField {
    const X_MIN: i32 = -260;
    const X_MAX: i32 = 740;

    pub fn new() -> Self {
        CubeField {
            cubes: [Cube::new(Position::new(0, 0), Velocity::new(0, 0)); TOTAL_CUBES as usize],
            player: Player::new()
        }
    }

    pub fn init(&mut self, rng: &mut SmallRng) {
        for row in 0..ROWS {
            let y = -((ROW_SPACE * row + 100) as i32);
            let delta = Position::new(0, y);

            for cube in 0..CUBES_PER_ROW {
                let i = (row * CUBES_PER_ROW + cube) as usize;
                self.cubes[i].0.translate(&delta);
                self.cubes[i].0.set_velocity(Velocity::new(0, CUBE_SPEED));
            }

            self.distribute_row(row as u32, rng);
        }
    }

    fn distribute_row(&mut self, row_number: u32, rng: &mut SmallRng) {
        assert!(row_number < ROWS as u32);

        let start_index: usize = (row_number * CUBES_PER_ROW as u32) as usize;
        let end_index: usize = start_index + CUBES_PER_ROW as usize;

        for i in start_index..end_index {
            let delta = Position::new(rng.gen_range(Self::X_MIN..Self::X_MAX), 0);
            self.cubes[i].0.translate(&delta);
        }
    }

    fn randomize_position(cube: &mut Cube, rng: &mut SmallRng) {
        let new_position = Position::new(
            rng.gen_range(Self::X_MIN..Self::X_MAX),
            cube.0.hit_box.top_left().y - (ROW_SPACE * ROWS) as i32
        );

        cube.0.set_position(&new_position);
    }

    pub fn process_frame(&mut self, lcd: &mut Lcd, dpad: &DPad, rng: &mut SmallRng) -> bool {
        let mut game_over = false;

        // lcd.set_color(RGB8::BLACK);

        let vx = match (dpad.left_pressed(), dpad.right_pressed()) {
            (false, false) => 0,
            (false, true) => -MOVEMENT_SPEED,
            (true, false) => MOVEMENT_SPEED,
            (true, true) => 0,
        };

        for cube in self.cubes.iter_mut() {
            cube.0.set_velocity(Velocity::new(vx, CUBE_SPEED));
            Self::update_cube_position(cube);

            if Self::object_on_screen(&cube.0) {
                if cube.0.hit_box.collides_with(&self.player.0.hit_box) {
                    game_over = true;
                    break;
                }

                handle_draw(cube.0.art.draw(lcd));
            } else if cube.0.hit_box.top_left().y > lcd::SCREEN_HEIGHT as i32 {
                Self::randomize_position(cube, rng)
            }
        }

        handle_draw(self.player.0.art.draw(lcd));

        game_over
    }
    
    fn object_on_screen<T>(obj: &MovingObject<T>) -> bool {
        let screen = BoundingBox::new(
            Position::new(LEFT_BOUND, TOP_BOUND), 
            Position::new(RIGHT_BOUND, BOTTOM_BOUND));

        screen.collides_with(&obj.hit_box)
    }

    fn update_cube_position(cube: &mut Cube) {
        cube.0.update_position();
        
        if cube.0.hit_box.top_left().x > Self::X_MAX {
            let y = cube.0.hit_box.top_left().y;
            cube.0.set_position(&Position::new(Self::X_MIN, y));
        } else if cube.0.hit_box.top_left().x < Self::X_MIN {
            let y = cube.0.hit_box.top_left().y;
            cube.0.translate(&Position::new(Self::X_MAX - Cube::SIDE_LENGTH, y));
        }
    }
}

#[derive(Clone, Copy)]
struct Cube(MovingObject<Rectangle>);

impl Cube {
    const SIDE_LENGTH: i32 = 50;

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
    const PLAYER_WIDTH: i32 = 50;
    const PLAYER_HEIGHT: i32 = 50;

    pub fn new() -> Self {
        let x_mid = (lcd::SCREEN_WIDTH / 2) as i32;
        let x_min = x_mid - (Self::PLAYER_WIDTH / 2);
        let x_max = x_min + Self::PLAYER_WIDTH;
        let y_min = (lcd::SCREEN_HEIGHT - 10) as i32;
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
