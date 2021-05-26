use defmt::Format;

use crate::common::Position;

pub trait Collideable<OTHER> {
    fn collides_with(&self, other: &OTHER) -> bool;
    fn collides_with_interpolate(&self, old_hit_box: &Self, other: &OTHER) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone, Copy, Debug, Format)]
pub struct BoundingBox {
    pub top_left: Position,
    pub bottom_right: Position,
}

impl BoundingBox {
    pub fn new(top_left: Position, bottom_right: Position) -> BoundingBox {
        BoundingBox {
            top_left,
            bottom_right,
        }
    }

    pub fn translate(&mut self, Position { x, y }: &Position) -> &BoundingBox {
        self.top_left.x += x;
        self.top_left.y += y;
        self.bottom_right.x += x;
        self.bottom_right.y += y;

        self
    }

    pub fn translate_to(&mut self, Position { x, y }: &Position) -> &BoundingBox {
        let delta = Position::new(x - self.top_left.x, y - self.top_left.y);

        self.top_left.x += delta.x;
        self.top_left.y += delta.y;
        self.bottom_right.x += delta.x;
        self.bottom_right.y += delta.y;

        self
    }

    pub fn width(&self) -> i32 {
        (self.top_left.x - self.bottom_right.x).abs()
    }

    pub fn height(&self) -> i32 {
        (self.top_left.y - self.bottom_right.y).abs()
    }

    pub fn center(&self) -> Position {
        Position::new(
            (self.top_left.x + self.bottom_right.x) / 2,
            (self.top_left.y + self.bottom_right.y) / 2,
        )
    }
}

impl Collideable<BoundingBox> for BoundingBox {
    fn collides_with(&self, other: &BoundingBox) -> bool {
        self.top_left.x < other.bottom_right.x
            && self.top_left.y < other.bottom_right.y
            && self.bottom_right.x > other.top_left.x
            && self.bottom_right.y > other.top_left.y
    }

    fn collides_with_interpolate(
        &self,
        old_hit_box: &Self,
        other: &BoundingBox,
    ) -> Option<BoundingBox> {
        let mut collision_location: Option<BoundingBox> = None;
        let dx: i32 = self.top_left.x - old_hit_box.top_left.x;
        let dy: i32 = self.top_left.y - old_hit_box.top_left.y;
        let steps: i32 = core::cmp::max(dx.abs(), dy.abs());

        if steps > 0 {
            for step in 0..=steps {
                let x_step: i32 = (step * dx) / steps;
                let y_step: i32 = (step * dy) / steps;

                let interpolated_hit_box = BoundingBox::new(
                    Position::new(
                        old_hit_box.top_left.x + x_step,
                        old_hit_box.top_left.y + y_step,
                    ),
                    Position::new(
                        old_hit_box.bottom_right.x + x_step,
                        old_hit_box.bottom_right.y + y_step,
                    ),
                );

                if interpolated_hit_box.collides_with(other) {
                    collision_location = Some(interpolated_hit_box);
                    break;
                }
            }
        }

        collision_location
    }
}
