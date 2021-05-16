use crate::common::{MovingObject, Position, Velocity};
use crate::images::{CubeImage, PlayerImage};
use crate::input::{DPad, DirectionalInput};
use crate::{
    collisions::{BoundingBox, Collideable},
    images::SimpleImage,
};
use lcd::{self, Lcd};
use rand::{rngs::SmallRng, Rng};
use stm32l4p5_hal::dma2d::Dma2d;

pub fn play(
    lcd: &mut Lcd,
    dpad: &DPad,
    dma2d: &mut Dma2d,
    rng: &mut SmallRng,
    draw_and_wait: fn() -> (),
) {
    let mut cube_field = CubeField::new();
    cube_field.randomize_field(rng);

    let mut game_over = false;

    while !game_over {
        game_over = cube_field.process_frame(dpad, dma2d, rng, RandomReposition);
        draw_and_wait();
    }

    dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);
}

const TOTAL_CUBES: u16 = 25;
const CUBES_PER_ROW: u16 = 5;
const ROWS: u16 = 5;
const ROW_SPACE: u16 = 150;
const BASE_CUBE_SPEED: i32 = 5;
const MOVEMENT_SPEED: i32 = 15;
const BACKGROUND_COLOR: u32 = 0xff_ff_ff_ff;
const QUARTER_WIDTH: u16 = 120;

type Cube = MovingObject<CubeImage>;
type Player = MovingObject<PlayerImage>;

struct CubeField {
    cubes: [Cube; TOTAL_CUBES as usize],
    player: Player,
    score: u32,
}

impl CubeField {
    const X_MIN: i32 = -260;
    const X_MAX: i32 = 740;

    pub fn new() -> Self {
        let cube_hit_box = BoundingBox::new(
            Position::new(0, 0),
            Position::new(CubeImage::WIDTH as i32, CubeImage::HEIGHT as i32),
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

        CubeField {
            cubes: [Cube::new(cube_hit_box, Velocity::default(), CubeImage); TOTAL_CUBES as usize],
            player: Player::new(player_hit_box, Velocity::default(), PlayerImage),
            score: 0
        }
    }

    pub fn randomize_field(&mut self, rng: &mut SmallRng) {
        for row in 0..ROWS {
            let y = -((ROW_SPACE * row + 100) as i32);
            let delta = Position::new(0, y);

            for cube in 0..CUBES_PER_ROW {
                let i = (row * CUBES_PER_ROW + cube) as usize;
                self.cubes[i].hit_box.translate(&delta);
                self.cubes[i].set_velocity(Velocity::new(0, BASE_CUBE_SPEED));
            }

            self.distribute_row(row as u32, rng);
        }
    }

    pub fn process_frame<R>(
        &mut self,
        dpad: &DPad,
        dma2d: &mut Dma2d,
        rng: &mut SmallRng,
        reposition_state: R
    ) -> bool 
    where
        R: CubeReposition,
    {
        dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);

        let mut game_over = false;

        let vx = match (dpad.left_pressed(), dpad.right_pressed()) {
            (false, false) => 0,
            (false, true) => -MOVEMENT_SPEED,
            (true, false) => MOVEMENT_SPEED,
            (true, true) => 0,
        };

        for cube in self.cubes.iter_mut() {
            cube.set_velocity(Velocity::new(vx, BASE_CUBE_SPEED));
            Self::update_cube_position(cube);

            if Self::object_on_screen(&cube.hit_box) {
                if cube.hit_box.collides_with(&self.player.hit_box) {
                    game_over = true;
                    break;
                }

                let cropped_box = BoundingBox::new(
                    Position::new(
                        cube.hit_box.top_left.x.clamp(0, lcd::SCREEN_WIDTH_I32),
                        cube.hit_box.top_left.y.clamp(0, lcd::SCREEN_HEIGHT_I32),
                    ),
                    Position::new(
                        cube.hit_box.bottom_right.x.clamp(0, lcd::SCREEN_WIDTH_I32),
                        cube.hit_box.bottom_right.y.clamp(0, lcd::SCREEN_HEIGHT_I32),
                    ),
                );

                let cropped_offset = cropped_box.top_left - cube.hit_box.top_left;

                let cropped_address: u32 = cube
                    .image
                    .data_address_offset(cropped_offset.x as u16, cropped_offset.y as u16);

                dma2d.draw_rgb8_image(
                    cropped_address,
                    cropped_box.top_left.x as u32,
                    cropped_box.top_left.y as u32,
                    cropped_box.width() as u16,
                    cropped_box.height() as u16,
                );
            } else if cube.hit_box.top_left.y > lcd::SCREEN_HEIGHT_I32 {
                reposition_state.reposition_cube(cube, rng);
                // Self::randomize_position(cube, rng)
            }
        }

        dma2d.draw_rgb8_image(
            self.player.image.data_address(),
            self.player.hit_box.top_left.x as u32,
            self.player.hit_box.top_left.y as u32,
            PlayerImage::WIDTH,
            PlayerImage::HEIGHT,
        );

        game_over
    }

    fn distribute_row(&mut self, row_number: u32, rng: &mut SmallRng) {
        assert!(row_number < ROWS as u32);

        let start_index: usize = (row_number * CUBES_PER_ROW as u32) as usize;
        let end_index: usize = start_index + CUBES_PER_ROW as usize;

        for i in start_index..end_index {
            let delta = Position::new(rng.gen_range(Self::X_MIN..Self::X_MAX), 0);
            self.cubes[i].hit_box.translate(&delta);
        }
    }

    fn randomize_position(cube: &mut Cube, rng: &mut SmallRng) {
        let new_position = Position::new(
            rng.gen_range(Self::X_MIN..Self::X_MAX),
            cube.hit_box.top_left.y - (ROW_SPACE * ROWS) as i32,
        );

        cube.hit_box.translate_to(&new_position);
    }

    fn object_on_screen(obj_hit_box: &BoundingBox) -> bool {
        let screen = BoundingBox::new(
            Position::new(0, 0),
            Position::new(lcd::SCREEN_WIDTH_I32, lcd::SCREEN_HEIGHT_I32),
        );

        screen.collides_with(obj_hit_box)
    }

    fn update_cube_position(cube: &mut Cube) -> &BoundingBox {
        cube.hit_box.translate(&cube.velocity).top_left;

        if cube.hit_box.top_left.x > Self::X_MAX {
            cube.hit_box
                .translate_to(&Position::new(Self::X_MIN, cube.hit_box.top_left.y))
        } else if cube.hit_box.top_left.x < Self::X_MIN {
            cube.hit_box.translate_to(&Position::new(
                Self::X_MAX - CubeImage::WIDTH as i32,
                cube.hit_box.top_left.y,
            ))
        } else {
            &cube.hit_box
        }
    }
}

trait CubeReposition {
    fn reposition_cube(&self, cube: &mut Cube, rng: &mut SmallRng);
}

struct RandomReposition;
impl CubeReposition for RandomReposition {
    fn reposition_cube(&self, cube: &mut Cube, rng: &mut SmallRng) {
        let new_position = Position::new(
            rng.gen_range(CubeField::X_MIN..CubeField::X_MAX),
            cube.hit_box.top_left.y - (ROW_SPACE * ROWS) as i32,
        );
    
        cube.hit_box.translate_to(&new_position);   
    }
}

struct StartTransition {
    
}