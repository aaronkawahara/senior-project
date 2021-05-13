use crate::common::Position;

pub trait Collideable<OTHER> {
    fn collides_with(&self, other: &OTHER) -> bool;
}

#[derive(Clone, Copy)]
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

    pub fn translate(&mut self, Position {x, y}: &Position) {
        self.top_left.x += x;
        self.top_left.y += y;
        self.bottom_right.x += x;
        self.bottom_right.y += y;
    }

    pub fn top_left(&self) -> &Position {
        &self.top_left
    }

    pub fn bottom_right(&self) -> &Position {
        &self.bottom_right
    }
}

impl Collideable<BoundingBox> for BoundingBox {
    fn collides_with(&self, other: &BoundingBox) -> bool {
        self.top_left.x < other.bottom_right.x &&
        self.top_left.y < other.bottom_right.y &&
        self.bottom_right.x > other.top_left.x &&
        self.bottom_right.y > other.top_left.y
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn bbox_translate_zero() {
//         let mut a: BoundingBox = BoundingBox::new(
//             Position::new(0, 0), 
//             Position::new(10, 10)
//         );

//         a.translate(&Position::new(0, 0));

//         assert_eq!(a.top_left, Position::new(0, 0));
//         assert_eq!(a.bottom_right, Position::new(10, 10));
//     }

//     #[test]
//     fn bbox_translate_positive() {
//         let mut a: BoundingBox = BoundingBox::new(
//             Position::new(0, 0), 
//             Position::new(10, 10)
//         );

//         a.translate(&Position {x: 10, y: 10});

//         assert_eq!(a.top_left, Position::new(10, 10));
//         assert_eq!(a.bottom_right, Position::new(20, 20));
//     }

//     #[test]
//     fn bbox_translate_negative() {
//         let mut a: BoundingBox = BoundingBox::new(
//             Position::new(0, 0), 
//             Position::new(10, 10)
//         );

//         a.translate(&Position {x: -10, y: -10});

//         assert_eq!(a.top_left, Position::new(-10, -10));
//         assert_eq!(a.bottom_right, Position::new(0, 0));
//     }

//     #[test]
//     fn bbox_collides_with_itself() {
//         let a: BoundingBox = BoundingBox::new(
//             Position::new(0,0), 
//             Position::new(10, 10)
//         );
        
//         assert_eq!(a.top_left, Position::new(0, 0));
//         assert_eq!(a.bottom_right, Position::new(10, 10));
//     }

//     #[test]
//     fn bbox_collides_with_other() {
//         let a: BoundingBox = BoundingBox::new(
//             Position::new(0,0), 
//             Position::new(10, 10)
//         );

//         let b: BoundingBox = BoundingBox::new(
//             Position::new(5,5), 
//             Position::new(15, 15)
//         );

//         assert!(a.collides_with(&b) && b.collides_with(&a));
//     }

//     #[test]
//     fn bbox_misses_other() {
//         let a: BoundingBox = BoundingBox::new(
//             Position::new(0,0), 
//             Position::new(10, 10)
//         );

//         let b: BoundingBox = BoundingBox::new(
//             Position::new(10, 0), 
//             Position::new(20, 10)
//         );

//         assert!(!a.collides_with(&b));
//     }
// }