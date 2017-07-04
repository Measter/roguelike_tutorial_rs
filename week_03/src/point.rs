use num::Num;

use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y
        }
    }
}

impl<T: Num> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, Point{x, y}: Point<T>) -> Point<T> {
        Point{
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<T: Num> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, Point{x, y}: Point<T>) -> Point<T> {
        Point{
            x: self.x - x,
            y: self.y - y,
        }
    }
}