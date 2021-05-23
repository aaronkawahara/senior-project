use crate::collisions::{BoundingBox, Collideable};
use crate::common::{MovingObject, Position, Velocity};
use crate::images::{self, OnlyLevelJustWallsImage, OnlyOneLevelPlayerImage, SimpleImage};
use crate::rng;

use super::levels::{self, JumpData};

use board::input::Inputs;
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
                Position::new(0, 0),
                Position::new(
                    OnlyOneLevelPlayerImage::WIDTH as i32,
                    OnlyOneLevelPlayerImage::HEIGHT as i32,
                ),
            ),
            Velocity::new(0, 0),
            OnlyOneLevelPlayerImage,
        );

        player.hit_box.translate(&Self::START_POSITION);

        OnlyLevel {
            level: 1,
            player,
            player_touching_ground: false,
            jump_data: JumpData::new(),
        }
    }

    pub fn process_frame(&mut self, input: &mut Inputs, dma2d: &mut Dma2d) -> Levels {
        let vx: i32 = match (input.left_pressed(), input.right_pressed()) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0,
        };

        let vy: i32 = if self.player_touching_ground && input.up_pressed() {
            self.jump_data.jump()
        } else if !self.player_touching_ground {
            self.jump_data.fall(&self.player.velocity.y)
        } else {
            self.jump_data.land()
        };

        let old_hit_box: BoundingBox = self.player.hit_box.clone();
        self.player.set_velocity(Velocity::new(vx, vy));
        self.player.hit_box.translate(&self.player.velocity);
        self.player_touching_ground = false;

        if self.player.hit_box.bottom_right.y >= lcd::SCREEN_HEIGHT_I32 {
            self.player.hit_box.translate(&Position::new(0, lcd::SCREEN_HEIGHT_I32 - self.player.hit_box.bottom_right.y));
            self.player_touching_ground = true;
        }

        for wall in WALL_HIT_BOXES.iter() {
            // if self.player.hit_box.collides_with(wall) {
            //     if self.player.hit_box.bottom_right.y > wall.top_left.y {
            //         self.player.hit_box.translate(&Position::new(
            //             0,
            //             self.player.hit_box.bottom_right.y - wall.top_left.y,
            //         ));
            //         self.player_touching_ground = true;
            //     } else if self.player.hit_box.top_left.y < wall.bottom_right.y {
            //         self.player.hit_box.translate(&Position::new(
            //             0,
            //             self.player.hit_box.top_left.y - wall.bottom_right.y,
            //         ));
            //     }

            //     if self.player.hit_box.bottom_right.x > wall.top_left.x {
            //         self.player.hit_box.translate(&Position::new(
            //             self.player.hit_box.bottom_right.x - wall.top_left.x,
            //             0,
            //         ));
            //     } else if self.player.hit_box.top_left.x < wall.bottom_right.x {
            //         self.player.hit_box.translate(&Position::new(
            //             self.player.hit_box.top_left.x - wall.bottom_right.x,
            //             0,
            //         ));
            //     }
            // }
        }

        dma2d.draw_rgb8_image(
            OnlyOneLevelPlayerImage.data_address(),
            self.player.hit_box.top_left.x as u32,
            self.player.hit_box.top_left.y as u32,
            OnlyOneLevelPlayerImage::WIDTH,
            OnlyOneLevelPlayerImage::HEIGHT,
        );

        self.level
    }
}

const WALL_HIT_BOXES: [BoundingBox; 23] = [
    BoundingBox {
        top_left: Position { x: 0, y: 0 },
        bottom_right: Position { x: 102, y: 68 },
    },
    BoundingBox {
        top_left: Position { x: 102, y: 0 },
        bottom_right: Position { x: 395, y: 16 },
    },
    BoundingBox {
        top_left: Position { x: 154, y: 16 },
        bottom_right: Position { x: 171, y: 34 },
    },
    BoundingBox {
        top_left: Position { x: 342, y: 16 },
        bottom_right: Position { x: 359, y: 34 },
    },
    BoundingBox {
        top_left: Position { x: 395, y: 0 },
        bottom_right: Position { x: 479, y: 50 },
    },
    BoundingBox {
        top_left: Position { x: 429, y: 50 },
        bottom_right: Position { x: 479, y: 84 },
    },
    BoundingBox {
        top_left: Position { x: 445, y: 84 },
        bottom_right: Position { x: 479, y: 102 },
    },
    BoundingBox {
        top_left: Position { x: 463, y: 102 },
        bottom_right: Position { x: 479, y: 239 },
    },
    BoundingBox {
        top_left: Position { x: 411, y: 152 },
        bottom_right: Position { x: 428, y: 171 },
    },
    BoundingBox {
        top_left: Position { x: 411, y: 171 },
        bottom_right: Position { x: 463, y: 204 },
    },
    BoundingBox {
        top_left: Position { x: 359, y: 239 },
        bottom_right: Position { x: 479, y: 479 },
    },
    BoundingBox {
        top_left: Position { x: 325, y: 137 },
        bottom_right: Position { x: 359, y: 153 },
    },
    BoundingBox {
        top_left: Position { x: 309, y: 69 },
        bottom_right: Position { x: 343, y: 85 },
    },
    BoundingBox {
        top_left: Position { x: 205, y: 119 },
        bottom_right: Position { x: 273, y: 136 },
    },
    BoundingBox {
        top_left: Position { x: 0, y: 68 },
        bottom_right: Position { x: 16, y: 153 },
    },
    BoundingBox {
        top_left: Position { x: 0, y: 153 },
        bottom_right: Position { x: 84, y: 187 },
    },
    BoundingBox {
        top_left: Position { x: 0, y: 187 },
        bottom_right: Position { x: 68, y: 271 },
    },
    BoundingBox {
        top_left: Position { x: 137, y: 170 },
        bottom_right: Position { x: 187, y: 187 },
    },
    BoundingBox {
        top_left: Position { x: 68, y: 255 },
        bottom_right: Position { x: 325, y: 271 },
    },
    BoundingBox {
        top_left: Position { x: 223, y: 187 },
        bottom_right: Position { x: 273, y: 221 },
    },
    BoundingBox {
        top_left: Position { x: 205, y: 221 },
        bottom_right: Position { x: 273, y: 255 },
    },
    BoundingBox {
        top_left: Position { x: 309, y: 204 },
        bottom_right: Position { x: 325, y: 238 },
    },
    BoundingBox {
        top_left: Position { x: 325, y: 221 },
        bottom_right: Position { x: 359, y: 271 },
    },
];
