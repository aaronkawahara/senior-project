use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, Velocity};
use crate::images::SimpleImage;
use crate::images::{CubeImage, PlayerImage};
use crate::input::{DPad, DirectionalInput};
use crate::rng;

use defmt;
use lcd::{self, Lcd};
use stm32l4p5_hal::dma2d::Dma2d;

pub fn play(lcd: &mut Lcd, dpad: &DPad, dma2d: &mut Dma2d, draw_and_wait: fn() -> ()) -> u32 {
    let mut cube_field = CubeField::new();
    let mut game_over = false;
    rng::init_rng();

    while !game_over {
        game_over = cube_field.process_frame(dpad, dma2d);
        draw_and_wait();
    }

    dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);
    cube_field.score
}

const TOTAL_CUBES: u16 = 30;
const CUBES_PER_ROW: u16 = 3;
const ROWS: u16 = 10;
const ROW_SPACE: u16 = 2 * CubeImage::HEIGHT;
const MOVEMENT_SPEED: i32 = 15;
const BACKGROUND_COLOR: u32 = 0xff_ff_ff_ff;
const QUARTER_WIDTH: u16 = 120;

type Cube = MovingObject<CubeImage>;
type Player = MovingObject<PlayerImage>;

struct CubeField {
    cubes: [Cube; TOTAL_CUBES as usize],
    player: Player,
    score: u32,
    zone: Zones,
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

        let mut cubes =
            [Cube::new(cube_hit_box, Velocity::default(), CubeImage); TOTAL_CUBES as usize];

        for row in 0..ROWS {
            let y: i32 = lcd::SCREEN_HEIGHT_I32 - PlayerImage::HEIGHT as i32 - (ROW_SPACE * row) as i32;

            for cube in 0..CUBES_PER_ROW {
                let x: i32 = match cube {
                    0 => StartZone::LEFT_COLUMN_X,
                    1 => StartZone::RIGHT_COLUMN_X,
                    _ => Self::X_MIN,
                };

                let i = (row * CUBES_PER_ROW + cube) as usize;
                cubes[i].hit_box.translate_to(&Position::new(x, y));
            }
        }

        CubeField {
            cubes,
            player: Player::new(player_hit_box, Velocity::default(), PlayerImage),
            score: 0,
            zone: Zones::Start(StartZone::default()),
        }
    }

    pub fn process_frame(&mut self, dpad: &DPad, dma2d: &mut Dma2d) -> bool {
        dma2d.fill_background(BACKGROUND_COLOR, QUARTER_WIDTH, lcd::SCREEN_HEIGHT_U16);

        let mut game_over = false;

        let vx: i32 = match (dpad.left_pressed(), dpad.right_pressed()) {
            (false, false) => 0,
            (false, true) => -MOVEMENT_SPEED,
            (true, false) => MOVEMENT_SPEED,
            (true, true) => 0,
        };

        let vy: i32 = self.zone.speed();

        for cube in self.cubes.iter_mut() {
            if cube.hit_box.collides_with(&self.player.hit_box) {
                game_over = true;
                break;
            }

            cube.set_velocity(Velocity::new(vx, vy));
            cube.hit_box.translate(&cube.velocity);

            if Self::object_on_screen(&cube.hit_box) {
                // TODO refactor the cropping logic to common or collisionss
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
            }

            if cube.hit_box.top_left.y > lcd::SCREEN_HEIGHT_I32 {
                self.zone.reposition_cube(cube);
            }

            Self::wrap_cube_if_out_of_bounds(cube);
        }

        dma2d.draw_rgb8_image(
            self.player.image.data_address(),
            self.player.hit_box.top_left.x as u32,
            self.player.hit_box.top_left.y as u32,
            PlayerImage::WIDTH,
            PlayerImage::HEIGHT,
        );

        self.score += vy as u32;

        if let Some(new_zone) = self.zone.next_zone(self.score) {
            self.zone = new_zone;
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

    fn wrap_cube_if_out_of_bounds(cube: &mut Cube) {
        if cube.hit_box.top_left.x > Self::X_MAX {
            cube.hit_box
                .translate_to(&Position::new(Self::X_MIN, cube.hit_box.top_left.y));
        } else if cube.hit_box.top_left.x < Self::X_MIN {
            cube.hit_box.translate_to(&Position::new(
                Self::X_MAX - CubeImage::WIDTH as i32,
                cube.hit_box.top_left.y,
            ));
        }
    }
}

enum Zones {
    Start(StartZone),
    Random(RandomizedZone),
    End(EndZone),
}

impl Zones {
    fn reposition_cube(&mut self, cube: &mut Cube) {
        match self {
            Zones::Start(z) => z.reposition_cube(cube),
            Zones::Random(z) => z.reposition_cube(cube),
            Zones::End(_) => {}
        }
    }

    fn next_zone(&self, score: u32) -> Option<Zones> {
        match self {
            Zones::Start(z) if z.passed_zone(score) => Some(z.next_zone()),
            Zones::Random(z) if z.passed_zone(score) => Some(z.next_zone()),
            Zones::End(_) => None,
            _ => None,
        }
    }

    fn speed(&self) -> i32 {
        match self {
            Zones::Start(_) => StartZone::CUBE_SPEED,
            Zones::Random(z) => z.cube_speed(),
            Zones::End(_) => 0,
        }
    }
}

trait ZoneBehavior {
    fn cube_speed(&self) -> i32;
    fn end(&self) -> u32;
    fn next_zone(&self) -> Zones;
    fn reposition_cube(&mut self, cube: &mut Cube);

    fn passed_zone(&self, score: u32) -> bool {
        score >= self.end()
    }
}

struct StartZone {
    left_count: u16,
    right_count: u16,
}

impl StartZone {
    const CUBE_SPEED: i32 = 5;
    const END: u32 = 1_500;
    const LEFT_COLUMN_X: i32 = lcd::SCREEN_WIDTH_I32 / 4 - (CubeImage::WIDTH as i32) / 2;
    const RIGHT_COLUMN_X: i32 = (lcd::SCREEN_WIDTH_I32 * 3) / 4 - (CubeImage::WIDTH as i32) / 2;

    fn default() -> Self {
        StartZone {
            left_count: ROWS,
            right_count: ROWS,
        }
    }
}

impl ZoneBehavior for StartZone {
    fn reposition_cube(&mut self, cube: &mut Cube) {
        cube.hit_box.translate(&Position::new(0, -((ROW_SPACE * ROWS) as i32)));
    }

    fn next_zone(&self) -> Zones {
        Zones::Random(RandomizedZone::new(Self::END))
    }

    fn cube_speed(&self) -> i32 {
        Self::CUBE_SPEED
    }

    fn end(&self) -> u32 {
        Self::END
    }
}

struct RandomizedZone {
    end: u32,
    level: i32,
}

impl RandomizedZone {
    const BASE_SPEED: i32 = 7;
    const SPEED_INCREMENT: i32 = 3;
    const BASE_ZONE_LENGTH: u32 = 5_000;

    pub fn new(zone_start: u32) -> Self {
        RandomizedZone {
            end: zone_start + Self::BASE_ZONE_LENGTH,
            level: 0,
        }
    }
}

impl ZoneBehavior for RandomizedZone {
    fn reposition_cube(&mut self, cube: &mut Cube) {
        let new_position = Position::new(
            rng::gen_range(CubeField::X_MIN..CubeField::X_MAX),
            cube.hit_box.top_left.y - (ROW_SPACE * ROWS) as i32,
        );

        cube.hit_box.translate_to(&new_position);
    }

    fn next_zone(&self) -> Zones {
        Zones::End(EndZone)
    }

    fn cube_speed(&self) -> i32 {
        Self::BASE_SPEED + (Self::SPEED_INCREMENT * self.level)
    }

    fn end(&self) -> u32 {
        self.end
    }
}

struct EndZone;
