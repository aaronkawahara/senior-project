use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, Velocity};
use crate::images::{
    OnlyLevelJustWallsImage, OnlyOneLevelPlayerBackgroundImage, OnlyOneLevelPlayerImage,
    SimpleImage,
};
use crate::rng;

use super::levels::JumpData;

use board::input::Inputs;
use defmt;
use lcd;
use stm32l4p5_hal::dma2d::Dma2d;

pub(crate) fn play(input: &mut Inputs, dma2d: &mut Dma2d, draw_and_wait: fn() -> ()) -> u32 {
    let mut level: Levels = 0;
    let mut only_level = OnlyLevel::new();
    rng::init_rng();

    dma2d.draw_rgb8_image(
        OnlyLevelJustWallsImage.data_address(),
        0,
        0,
        OnlyLevelJustWallsImage::WIDTH,
        OnlyLevelJustWallsImage::HEIGHT,
    );

    while level <= OnlyLevel::LAST_LEVEL {
        level = only_level.process_frame(input, dma2d);
        draw_and_wait();
    }

    dma2d.fill_background(
        0x00_00_00_00,
        lcd::SCREEN_WIDTH_U16 / 4,
        lcd::SCREEN_HEIGHT_U16,
    );

    0
}

type Player = MovingObject<OnlyOneLevelPlayerImage>;
type Levels = u8;

struct OnlyLevel {
    level: Levels,
    player: Player,
    player_touching_ground: bool,
    jump_data: JumpData,
}

impl OnlyLevel {
    const LAST_LEVEL: Levels = 1;
    const START_POSITION: Position = Position { x: 50, y: 80 };

    pub fn new() -> Self {
        let mut player: Player = Player::new(
            BoundingBox::new(
                Position::new(1, 1),
                Position::new(
                    OnlyOneLevelPlayerImage::WIDTH as i32,
                    OnlyOneLevelPlayerImage::HEIGHT as i32,
                ),
            ),
            Velocity::new(0, 0),
            OnlyOneLevelPlayerImage,
        );

        player.hit_box.translate_to(&Self::START_POSITION);

        OnlyLevel {
            level: 1,
            player,
            player_touching_ground: false,
            jump_data: JumpData::new(),
        }
    }

    pub fn process_frame(&mut self, input: &mut Inputs, dma2d: &mut Dma2d) -> Levels {
        let vx: i32 = match (input.left_pressed(), input.right_pressed()) {
            (true, false) => -3,
            (false, true) => 3,
            _ => 0,
        };

        let vy: i32 = if !self.player_touching_ground {
            self.jump_data.fall(&self.player.velocity.y)
        } else if input.up_pressed() {
            self.jump_data.jump()
        } else {
            0
        };

        dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerBackgroundImage.data_address(),
            core::cmp::max(0, self.player.hit_box.top_left.x) as u32,
            core::cmp::max(0, self.player.hit_box.top_left.y) as u32,
            OnlyOneLevelPlayerBackgroundImage::WIDTH,
            OnlyOneLevelPlayerBackgroundImage::HEIGHT,
        );

        let old_hit_box: BoundingBox = self.player.hit_box.clone();
        self.player.set_velocity(Velocity::new(vx, vy));
        self.player.hit_box.translate(&self.player.velocity);
        self.player_touching_ground = false;

        for wall in WALL_HIT_BOXES.iter() {
            if let Some(collision_location) = self
                .player
                .hit_box
                .collides_with_interpolate(&old_hit_box, wall)
            {
                self.player
                    .push_out_of(&old_hit_box, collision_location, wall);
            }

            if wall.top_left.x < self.player.hit_box.bottom_right.x
                && wall.bottom_right.x > self.player.hit_box.top_left.x
            {
                if self.player.hit_box.bottom_right.y == wall.top_left.y {
                    self.player_touching_ground = true;
                }
            }

            if self.player.hit_box.collides_with(wall) {
                defmt::info!("old player: {}", old_hit_box);
                defmt::info!("curr player: {}", self.player.hit_box);
                defmt::info!("wall: {}", wall);
                panic!();
            }
        }

        dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerImage.data_address(),
            core::cmp::max(0, self.player.hit_box.top_left.x) as u32,
            core::cmp::max(0, self.player.hit_box.top_left.y) as u32,
            OnlyOneLevelPlayerImage::WIDTH,
            OnlyOneLevelPlayerImage::HEIGHT,
        );

        self.level
    }
}

const WALL_HIT_BOXES: [BoundingBox; 23] = [
    BoundingBox {
        top_left: Position { x: 0, y: 0 },
        bottom_right: Position { x: 103, y: 69 },
    },
    BoundingBox {
        top_left: Position { x: 101, y: 0},
        bottom_right: Position { x: 396, y: 17 },
    },
    BoundingBox {
        top_left: Position { x: 153, y: 15 },
        bottom_right: Position { x: 172, y: 35 },
    },
    BoundingBox {
        top_left: Position { x: 341, y: 15},
        bottom_right: Position { x: 360, y: 35 },
    },
    BoundingBox {
        top_left: Position { x: 394, y: 0 },
        bottom_right: Position { x: 480, y: 51 },
    },
    BoundingBox {
        top_left: Position { x: 430, y: 49 },
        bottom_right: Position { x: 480, y: 85 },
    },
    BoundingBox {
        top_left: Position { x: 444, y: 83 },
        bottom_right: Position { x: 480, y: 103 },
    },
    BoundingBox {
        top_left: Position { x: 462, y: 101 },
        bottom_right: Position { x: 480, y: 240 },
    },
    BoundingBox {
        top_left: Position { x: 410, y: 151 },
        bottom_right: Position { x: 429, y: 172 },
    },
    BoundingBox {
        top_left: Position { x: 410, y: 170 },
        bottom_right: Position { x: 464, y: 205 },
    },
    BoundingBox {
        top_left: Position { x: 358, y: 238 },
        bottom_right: Position { x: 480, y: 480 },
    },
    BoundingBox {
        top_left: Position { x: 324, y: 136 },
        bottom_right: Position { x: 360, y: 154 },
    },
    BoundingBox {
        top_left: Position { x: 308, y: 68 },
        bottom_right: Position { x: 344, y: 86 },
    },
    BoundingBox {
        top_left: Position { x: 204, y: 118 },
        bottom_right: Position { x: 274, y: 137 },
    },
    BoundingBox {
        top_left: Position { x: 0, y: 67 },
        bottom_right: Position { x: 17, y: 154 },
    },
    BoundingBox {
        top_left: Position { x: 0, y: 152 },
        bottom_right: Position { x: 85, y: 188 },
    },
    BoundingBox {
        top_left: Position { x: 0, y: 186 },
        bottom_right: Position { x: 69, y: 272 },
    },
    BoundingBox {
        top_left: Position { x: 136, y: 169 },
        bottom_right: Position { x: 188, y: 188 },
    },
    BoundingBox {
        top_left: Position { x: 67, y: 254 },
        bottom_right: Position { x: 326, y: 272 },
    },
    BoundingBox {
        top_left: Position { x: 222, y: 186 },
        bottom_right: Position { x: 274, y: 222 },
    },
    BoundingBox {
        top_left: Position { x: 204, y: 220 },
        bottom_right: Position { x: 274, y: 256 },
    },
    BoundingBox {
        top_left: Position { x: 308, y: 203 },
        bottom_right: Position { x: 326, y: 239 },
    },
    BoundingBox {
        top_left: Position { x: 324, y: 220 },
        bottom_right: Position { x: 360, y: 272 },
    },
];
