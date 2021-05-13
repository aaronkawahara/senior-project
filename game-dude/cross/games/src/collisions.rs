use crate::common::Position;

pub trait Collideable<OTHER> {
    fn collides_with(&self, other: &OTHER) -> bool;
}

#[derive(Clone, Copy)]
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
}

impl Collideable<BoundingBox> for BoundingBox {
    fn collides_with(&self, other: &BoundingBox) -> bool {
        self.top_left.x < other.bottom_right.x
            && self.top_left.y < other.bottom_right.y
            && self.bottom_right.x > other.top_left.x
            && self.bottom_right.y > other.top_left.y
    }
}
