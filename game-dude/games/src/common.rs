use crate::collisions::BoundingBox;
use crate::graphics::{Point, Primitive, PrimitiveStyle, Styled, Transform};
use lcd::RGB8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x,
            y,
        }
    }

    pub fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }
}

impl From<&Position> for Point {
    fn from(p: &Position) -> Self {
        Point::new(p.x, p.y)
    }
}

pub type Velocity = Position;

pub type PrimitiveArt<G> = Styled<G, PrimitiveStyle<RGB8>>;

#[derive(Clone, Copy)]
pub struct MovingObject<S> {
    pub(crate) hit_box: BoundingBox,
    pub(crate) velocity: Velocity,
    pub(crate) art: PrimitiveArt<S>,
}

impl<S> MovingObject<S>
where 
    S: Transform,
{
    pub fn move_to_origin(&mut self) {
        let negated_x = -self.hit_box.top_left().x;
        let negated_y = -self.hit_box.top_left().y;
        let negated_posn = Position::new(negated_x, negated_y);

        self.hit_box.translate(&negated_posn);
        self.art.translate_mut(Point::from(&negated_posn));
    }

    pub fn translate(&mut self, delta: &Position) {
        self.hit_box.translate(delta);
        self.art.translate_mut(Point::from(delta));
    }

    pub fn set_position(&mut self, new_position: &Position) {
        let delta = Position::new(
            new_position.x - self.hit_box.top_left().x, 
            new_position.y - self.hit_box.top_left().y
        );

        self.hit_box.translate(&delta);
        self.art.translate_mut(Point::from(&delta));
    }

    pub fn update_position(&mut self) {
        self.hit_box.translate(&self.velocity);
        self.art.translate_mut(Point::from(&self.velocity));
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }
}
