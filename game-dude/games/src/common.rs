pub trait Collideable<OTHER> {
    fn collides_with(&self, other: OTHER) -> bool;
}

pub struct Position {
    x: i32,
    y: i32,
}

pub struct BoundingBox {
    top_left: Position,
    bottom_right: Position,
}

impl BoundingBox {
    pub fn new(top_left: Position, bottom_right: Position) -> BoundingBox {
        BoundingBox {
            top_left,
            bottom_right,
        }
    }

    pub fn translate(&mut self, Position {x, y}: Position) {
        self.top_left.x += x;
        self.top_left.y += y;
        self.bottom_right.x += x;
        self.bottom_right.y += y;
    }
}

impl Collideable<BoundingBox> for BoundingBox {
    fn collides_with(&self, other: BoundingBox) -> bool {
        self.bottom_right.x >= other.top_left.x && self.bottom_right.y >= other.top_left.y ||
        self.top_left.x <= other.bottom_right.x && self.top_left.y <= other.bottom_right.y
    }
}
