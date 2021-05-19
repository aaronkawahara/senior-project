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

const TOTAL_CUBES: u16 = ROWS * CUBES_PER_ROW;
const CUBES_PER_ROW: u16 = 4;
const ROWS: u16 = lcd::SCREEN_HEIGHT_U16 / ROW_SPACE + 2; // 1 for rounding up and 1 for smoothness
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
    const X_MIN: i32 = -lcd::SCREEN_WIDTH_I32;
    const X_MAX: i32 = lcd::SCREEN_WIDTH_I32 * 2;

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

        let zone = Zones::Empty(EmptyZone::default().setup(&mut cubes));

        CubeField {
            cubes,
            player: Player::new(player_hit_box, Velocity::default(), PlayerImage),
            score: 0,
            zone,
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

        dma2d.draw_rgb8_image(
            self.player.image.data_address(),
            self.player.hit_box.top_left.x as u32,
            self.player.hit_box.top_left.y as u32,
            PlayerImage::WIDTH,
            PlayerImage::HEIGHT,
        );

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

        self.score += vy as u32;

        let zone = core::mem::take(&mut self.zone);
        if let Some(new_zone) = zone.next_zone(self.score, &mut self.cubes) {
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

    fn wrap_cube_if_out_of_bounds(cube: &mut Cube) {
        if cube.hit_box.top_left.x > Self::X_MAX {
            cube.hit_box
                .translate_to(&Position::new(Self::X_MIN, cube.hit_box.top_left.y));
        } else if cube.hit_box.bottom_right.x < Self::X_MIN {
            cube.hit_box.translate_to(&Position::new(
                Self::X_MAX - CubeImage::WIDTH as i32,
                cube.hit_box.top_left.y,
            ));
        }
    }
}

enum Zones {
    Empty(EmptyZone),
    Transition(TransitionZone),
    Random(RandomizedZone),
    End(EndZone),
}

impl Default for Zones {
    fn default() -> Self {
        Zones::End(EndZone)
    }
}

impl Zones {
    fn reposition_cube(&mut self, cube: &mut Cube) {
        match self {
            Zones::Empty(_) => {}
            Zones::Transition(z) => z.reposition_cube(cube),
            Zones::Random(z) => z.reposition_cube(cube),
            Zones::End(_) => {}
        }
    }

    fn next_zone(self, score: u32, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Option<Zones> {
        match self {
            Zones::Empty(z) if z.passed_zone(score) => Some(z.next_zone(cubes)),
            Zones::Transition(z) if z.passed_zone(score) => Some(z.next_zone(cubes)),
            Zones::Random(z) if z.passed_zone(score) => Some(z.next_zone(cubes)),
            Zones::End(_) => None,
            _zone => Some(_zone),
        }
    }

    fn speed(&self) -> i32 {
        match self {
            Zones::Empty(z) => z.cube_speed(),
            Zones::Transition(z) => z.cube_speed(),
            Zones::Random(z) => z.cube_speed(),
            Zones::End(_) => 0,
        }
    }
}

trait ZoneBehavior {
    fn cube_speed(&self) -> i32;
    fn end(&self) -> u32;
    fn next_zone(self, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Zones;
    fn reposition_cube(&mut self, cube: &mut Cube);
    fn setup(self, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Self;

    fn passed_zone(&self, score: u32) -> bool {
        score >= self.end()
    }
}

struct EmptyZone {
    end: u32,
    level_data: LevelData,
    transition_next: bool,
}

impl EmptyZone {
    const ZONE_LENGTH: u32 = 2 * lcd::SCREEN_HEIGHT_U32;

    fn default() -> Self {
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
    fn cube_speed(&self) -> i32 {
        self.level_data.cube_speed
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn next_zone(self, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Zones {
        if self.transition_next {
            Zones::Transition(TransitionZone::new(self.level_data, self.end).setup(cubes))
        } else {
            Zones::Random(
                RandomizedZone::new(self.level_data, self.end + lcd::SCREEN_HEIGHT_U32).setup(cubes),
            )
        }
        
    }

    fn reposition_cube(&mut self, _cube: &mut Cube) {}

    fn setup(self, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Self {
        for cube in cubes.iter_mut() {
            cube.hit_box
                .translate_to(&Position::new(0, lcd::SCREEN_WIDTH_I32));
        }

        self
    }
}

struct TransitionZone {
    curr_cube: u16,
    end: u32,
    level_data: LevelData,
    rows_passed: i32,
}

impl TransitionZone {
    const BASE_SPEED: i32 = 5;
    const SPEED_INCREMENT: i32 = 2;
    const DX: i32 = CubeImage::WIDTH as i32;
    const LEFT_WALL_START: i32 = -(lcd::SCREEN_WIDTH_I32 / 2) + (CubeImage::WIDTH as i32) / 2;
    const LEFT_WALL_END: i32 = lcd::SCREEN_WIDTH_I32 / 4 - (CubeImage::WIDTH as i32) / 2;
    const RIGHT_WALL_START: i32 = (lcd::SCREEN_WIDTH_I32 + lcd::SCREEN_WIDTH_I32 / 2)
        - (CubeImage::WIDTH as i32) / 2
        - CubeImage::WIDTH as i32;
    const RIGHT_WALL_END: i32 = (lcd::SCREEN_WIDTH_I32 * 3) / 4;
    const CORRAL_ROWS: i32 = (Self::RIGHT_WALL_START - Self::RIGHT_WALL_END) / Self::DX + 1;
    const RUNWAY_ROWS: i32 = 2 * ROWS as i32;
    const TRANSITION_ROWS: i32 = Self::CORRAL_ROWS + Self::RUNWAY_ROWS;
    const TRANSITION_LENGTH: u32 = Self::TRANSITION_ROWS as u32 * ROW_SPACE as u32;

    fn default() -> Self {
        TransitionZone {
            curr_cube: 0,
            end: Self::TRANSITION_LENGTH,
            level_data: LevelData::default(),
            rows_passed: 0,
        }
    }

    fn new(mut level_data: LevelData, zone_start: u32) -> Self {
        level_data.cube_speed = Self::BASE_SPEED + (Self::SPEED_INCREMENT * level_data.level);

        TransitionZone {
            curr_cube: 0,
            end: zone_start + Self::TRANSITION_LENGTH,
            level_data,
            rows_passed: 0,
        }
    }
}

impl ZoneBehavior for TransitionZone {
    fn reposition_cube(&mut self, cube: &mut Cube) {
        let x: i32 = match self.curr_cube {
            0 => Self::DX * (Self::CORRAL_ROWS - self.rows_passed).clamp(0, ROWS as i32),
            1 => -Self::DX * (Self::CORRAL_ROWS - self.rows_passed).clamp(0, ROWS as i32),
            _ => 0,
        };

        let y: i32 = -((ROW_SPACE * ROWS) as i32);

        cube.hit_box.translate(&Position::new(x, y));

        self.curr_cube += 1;
        if self.curr_cube == CUBES_PER_ROW {
            self.rows_passed += 1;
            self.curr_cube = 0;
        }
    }

    fn next_zone(self, _cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Zones {
        Zones::Empty(EmptyZone::new(self.level_data, self.end, false))
    }

    fn cube_speed(&self) -> i32 {
        self.level_data.cube_speed
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn setup(self, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Self {
        for row in 0..(ROWS as i32) {
            let y: i32 = lcd::SCREEN_HEIGHT_I32 - (CubeImage::HEIGHT as i32 + (ROW_SPACE as i32) * row);

            for cube in 0..CUBES_PER_ROW {
                let x: i32 = match cube {
                    0 => Self::LEFT_WALL_START + row * Self::DX,
                    1 => Self::RIGHT_WALL_START - row * Self::DX,
                    _ => CubeField::X_MIN,
                };

                let i = (row as u16 * CUBES_PER_ROW + cube) as usize;
                cubes[i].hit_box.translate_to(&Position::new(x, y));
            }
        }

        self
    }
}

struct RandomizedZone {
    end: u32,
    level_data: LevelData,
}

impl RandomizedZone {
    const BASE_ZONE_LENGTH: u32 = 1_000;
    const ZONE_INCREMENT: u32 = TransitionZone::SPEED_INCREMENT as u32 * 1000;

    pub fn new(level_data: LevelData, zone_start: u32) -> Self {
        RandomizedZone {
            end: zone_start
                + (Self::BASE_ZONE_LENGTH + Self::ZONE_INCREMENT * level_data.level as u32),
            level_data,
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

    fn next_zone(mut self, _cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Zones {
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

    fn cube_speed(&self) -> i32 {
        self.level_data.cube_speed
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn setup(self, cubes: &mut [Cube; TOTAL_CUBES as usize]) -> Self {
        for row in 0..(ROWS as i32) {
            let y: i32 = -(CubeImage::HEIGHT as i32 + (ROW_SPACE as i32) * row);

            for cube in 0..CUBES_PER_ROW {
                let x: i32 = rng::gen_range(CubeField::X_MIN..CubeField::X_MAX);

                let i = (row as u16 * CUBES_PER_ROW + cube) as usize;
                cubes[i].hit_box.translate_to(&Position::new(x, y));
            }
        }

        self
    }
}

struct EndZone;

struct LevelData {
    cube_speed: i32,
    level: i32,
}

impl LevelData {
    const START_SPEED: i32 = 5;

    pub fn default() -> Self {
        LevelData {
            cube_speed: Self::START_SPEED,
            level: 0,
        }
    }
}
