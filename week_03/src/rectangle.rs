use point::Point;

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point<i8>,
    pub bottom_right: Point<i8>,
}

impl Rectangle {
    pub fn new(top_left: Point<i8>, (width, height): (u8, u8)) -> Rectangle {
        Rectangle {
            top_left: top_left,
            bottom_right: Point {
                x: top_left.x + width as i8,
                y: top_left.y + height as i8,
            }
        }
    }

    pub fn centre(&self) -> Point<i8> {
        Point {
            x: ((self.top_left.x as i16 + self.bottom_right.x as i16) / 2) as i8,
            y: ((self.top_left.y as i16 + self.bottom_right.y as i16) / 2) as i8,
        }
    }

    pub fn is_intersecting(&self, other: &Rectangle) -> bool {
        self.top_left.x <= other.bottom_right.x && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y && self.bottom_right.y >= other.top_left.y
    }
}