pub trait Point {
    fn lerp(&self, other: Self, t: f32) -> Self;
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct TwoDimensionalPoint {
    x: f32,
    y: f32
}

impl TwoDimensionalPoint {
    pub fn new(x: f32, y: f32) -> Self {
        TwoDimensionalPoint {
            x,
            y
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
}

impl Point for TwoDimensionalPoint {
    fn lerp(&self, other: Self, t: f32) -> Self {
        let mut x = 0.0;
        let mut y = 0.0;

        if other.x != self.x {
            x = other.x - (t * (other.x - self.x));
            y = (self.y * (other.x - x) + other.y * (x - self.x)) / (other.x - self.x);
        } else {
            x = self.x;
            y = other.y - ((other.y - self.y) * t);
        }

        TwoDimensionalPoint {
            x,
            y
        }
    }
}

pub struct ThreeDimensionalPoint {
    x: f32,
    y: f32,
    z: f32
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::TwoDimensionalPoint;

    #[test]
    fn test_2d_lerp() {
        let a = TwoDimensionalPoint { x: 0.0, y: 0.0 };
        let b = TwoDimensionalPoint { x: 1.0, y: 1.0 };

        let res = a.lerp(b, 0.5);

        assert_eq!(res, TwoDimensionalPoint { x: 0.5, y: 0.5 });
    }
}