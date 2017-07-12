use point::Point;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Rectangle {
    pub top_left: Point<i16>,
    pub bottom_right: Point<i16>,
}

impl Rectangle {
    pub fn new(top_left: Point<i16>, (width, height): (u8, u8)) -> Rectangle {
        Rectangle {
            top_left: top_left,
            bottom_right: Point {
                x: top_left.x + width as i16,
                y: top_left.y + height as i16,
            }
        }
    }

    pub fn centre(&self) -> Point<i16> {
        Point {
            x: ((self.top_left.x + self.bottom_right.x) / 2),
            y: ((self.top_left.y + self.bottom_right.y) / 2),
        }
    }

    pub fn is_intersecting(&self, other: &Rectangle) -> bool {
        self.top_left.x <= other.bottom_right.x && self.bottom_right.x >= other.top_left.x
            && self.top_left.y <= other.bottom_right.y && self.bottom_right.y >= other.top_left.y
    }

    pub fn clamp_to(&mut self, (left, top): (i16, i16), (right, bottom): (i16, i16)) {
        if self.top_left.x < left {
            let diff = left - self.top_left.x;
            self.top_left.x += diff;
            self.bottom_right.x += diff;
        }
        if self.top_left.y < top {
            let diff = top - self.top_left.y;
            self.top_left.y += diff;
            self.bottom_right.y += diff;
        }

        if self.bottom_right.x > right {
            let diff = right - self.bottom_right.x;
            self.top_left.x += diff;
            self.bottom_right.x += diff;
        }
        if self.bottom_right.y > bottom {
            let diff = bottom - self.bottom_right.y;
            self.top_left.y += diff;
            self.bottom_right.y += diff;
        }
    }
}