use crate::maneatingape::point::*;

#[inline]
pub fn direction_to_index(direction: Point) -> usize {
    match direction {
        LEFT => 0,
        RIGHT => 1,
        UP => 2,
        DOWN => 3,
        _ => unreachable!(),
    }
}

#[inline]
pub fn index_to_direction(index: usize) -> Point {
    match index {
        0 => LEFT,
        1 => RIGHT,
        2 => UP,
        3 => DOWN,
        _ => unreachable!(),
    }
}
