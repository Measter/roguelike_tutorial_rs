use num::{Num, ToPrimitive};

use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point<T: Num + ToPrimitive> {
    pub x: T,
    pub y: T,
}

impl<T: Num + ToPrimitive> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {
            x: x,
            y: y
        }
    }

    pub fn sqr_radius(&self) -> f64 {
        let x = self.x.to_f64().expect("Failed to convert to float.");
        let y = self.y.to_f64().expect("Failed to convert to float.");
        x*x + y*y
    }
}

impl<T: Num + ToPrimitive> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, Point{x, y}: Point<T>) -> Point<T> {
        Point{
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<T: Num + ToPrimitive> Sub for Point<T> {
    type Output = Point<T>;
    fn sub(self, Point{x, y}: Point<T>) -> Point<T> {
        Point{
            x: self.x - x,
            y: self.y - y,
        }
    }
}