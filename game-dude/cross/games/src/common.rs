use crate::collisions::BoundingBox;
use crate::images::SimpleImage;
use core::ops::{Add, Sub};
use defmt::{self, Format};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Format)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: Self) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub type Velocity = Position;

#[derive(Clone, Copy)]
pub struct MovingObject<I>
where
    I: SimpleImage,
{
    pub(crate) hit_box: BoundingBox,
    pub(crate) velocity: Velocity,
    pub(crate) image: I,
}

impl<I: SimpleImage> MovingObject<I> {
    pub fn new(hit_box: BoundingBox, velocity: Velocity, image: I) -> Self {
        MovingObject {
            hit_box,
            velocity,
            image,
        }
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn push_out_of(
        &mut self,
        &old_hit_box: &BoundingBox,
        mut collision_location: BoundingBox,
        other_hit_box: &BoundingBox,
    ) {
        if self.velocity.x > 0 && other_hit_box.top_left.x >= old_hit_box.bottom_right.x {
            collision_location.translate(&Position::new(
                other_hit_box.top_left.x - collision_location.bottom_right.x,
                0,
            ));
            self.velocity.x = 0;
        } else if self.velocity.x < 0 && other_hit_box.bottom_right.x <= old_hit_box.top_left.x {
            collision_location.translate(&Position::new(
                other_hit_box.bottom_right.x - collision_location.top_left.x,
                0,
            ));
            self.velocity.x = 0;
        } else if self.velocity.y > 0 {
            collision_location.translate(&Position::new(
                0,
                other_hit_box.top_left.y - collision_location.bottom_right.y,
            ));
            self.velocity.y = 0;
        } else if self.velocity.y < 0 {
            collision_location.translate(&Position::new(
                0,
                other_hit_box.bottom_right.y - collision_location.top_left.y,
            ));
            self.velocity.y = 0;
        }

        self.hit_box = collision_location;
    }
}
