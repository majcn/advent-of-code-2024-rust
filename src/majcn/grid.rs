use crate::maneatingape::grid::*;
use crate::maneatingape::point::*;

pub trait GridPointsIterator {
    fn points(&self) -> GridPointsIter;
}

pub struct GridPointsIter {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl<T> GridPointsIterator for Grid<T> {
    fn points(&self) -> GridPointsIter {
        GridPointsIter {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }
}

impl Iterator for GridPointsIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }

        if self.y == self.height {
            return None;
        }

        self.x += 1;

        Some(Point::new(self.x - 1, self.y))
    }
}
