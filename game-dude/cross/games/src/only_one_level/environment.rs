use crate::collisions::BoundingBox;
use crate::common::Position;
use crate::images::OnlyLevelButtonImage;
use crate::images::OnlyLevelEnvironmentImage;
use crate::images::OnlyLevelFinishPipeImage;
use crate::images::OnlyOneLevelPlayerImage;
use crate::images::SimpleImage;

use stm32l4p5_hal::dma2d::Dma2d;

pub(super) fn draw_environment(dma2d: &mut Dma2d) {
    dma2d.draw_rgb8_image(
        OnlyLevelEnvironmentImage.data_address(),
        0,
        0,
        OnlyLevelEnvironmentImage::WIDTH,
        OnlyLevelEnvironmentImage::HEIGHT,
    );
}

const SPAWN_TOP_MIDDLE: Position = Position { x: 74, y: 91 };
pub(super) const SPAWN_TOP_LEFT: Position = Position {
    x: SPAWN_TOP_MIDDLE.x - (OnlyOneLevelPlayerImage::WIDTH / 2) as i32,
    y: SPAWN_TOP_MIDDLE.y,
};

pub(super) const BUTTON_TOP_LEFT: Position = Position { x: 233, y: 111 };
pub(super) const BUTTON_HIT_BOX: BoundingBox = BoundingBox {
    top_left: Position {
        x: BUTTON_TOP_LEFT.x - 1,
        y: BUTTON_TOP_LEFT.y - 1,
    },
    bottom_right: Position {
        x: BUTTON_TOP_LEFT.x + OnlyLevelButtonImage::WIDTH as i32 + 1,
        y: BUTTON_TOP_LEFT.y + OnlyLevelButtonImage::HEIGHT as i32 + 1,
    },
};

pub(super) const START_PIPE_TOP_LEFT: Position = Position { x: 62, y: 69 };
pub(super) const FINISH_PIPE_TOP_LEFT: Position = Position { x: 442, y: 218 };
pub(super) const FINISH_PIPE_HIT_BOX: BoundingBox = BoundingBox {
    top_left: Position {
        x: FINISH_PIPE_TOP_LEFT.x - 1,
        y: FINISH_PIPE_TOP_LEFT.y - 1,
    },
    bottom_right: Position {
        x: FINISH_PIPE_TOP_LEFT.x + OnlyLevelFinishPipeImage::WIDTH as i32 + 1,
        y: FINISH_PIPE_TOP_LEFT.y + OnlyLevelFinishPipeImage::HEIGHT as i32 + 1,
    },
};

pub(super) const WALL_HIT_BOXES: [BoundingBox; 23] = [
    BoundingBox {
        top_left: Position { x: 0, y: 0 },
        bottom_right: Position { x: 103, y: 69 },
    },
    BoundingBox {
        top_left: Position { x: 101, y: 0 },
        bottom_right: Position { x: 396, y: 17 },
    },
    BoundingBox {
        top_left: Position { x: 153, y: 15 },
        bottom_right: Position { x: 172, y: 35 },
    },
    BoundingBox {
        top_left: Position { x: 341, y: 15 },
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

pub(super) const SPIKE_HIT_BOXES: [BoundingBox; 7] = [
    BoundingBox {
        top_left: Position { x: 16, y: 102 },
        bottom_right: Position { x: 30, y: 151 },
    },
    BoundingBox {
        top_left: Position { x: 68, y: 242 },
        bottom_right: Position { x: 118, y: 255 },
    },
    BoundingBox {
        top_left: Position { x: 275, y: 244 },
        bottom_right: Position { x: 325, y: 255 },
    },
    BoundingBox {
        top_left: Position { x: 360, y: 225 },
        bottom_right: Position { x: 374, y: 239 },
    },
    BoundingBox {
        top_left: Position { x: 449, y: 104 },
        bottom_right: Position { x: 463, y: 169 },
    },
    BoundingBox {
        top_left: Position { x: 400, y: 50 },
        bottom_right: Position { x: 429, y: 64 },
    },
    BoundingBox {
        top_left: Position { x: 225, y: 16 },
        bottom_right: Position { x: 282, y: 30 },
    },
];
