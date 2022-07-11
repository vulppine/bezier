use std::fmt::Error;
use crate::point::Point;
use crate::TwoDimensionalPoint;

pub struct QuadraticCurve {
    curve: Curve<TwoDimensionalPoint>,
}

impl QuadraticCurve {
    pub fn new(p0: TwoDimensionalPoint, p1: TwoDimensionalPoint, p2: TwoDimensionalPoint, p3: TwoDimensionalPoint) -> Self {
        let mut curve = Curve::<TwoDimensionalPoint>::new();

        curve.add_point(p0);
        curve.add_point(p1);
        curve.add_point(p2);
        curve.add_point(p3);

        Self {
            curve
        }
    }

    pub fn iter(&self, step: f32) -> Iter<TwoDimensionalPoint> {
        self.curve.iter(step)
    }

    pub fn set_point(&mut self, i: usize, point: TwoDimensionalPoint) -> Result<(), String> {
        // TODO: Actual errors
        if !self.curve.set_point(i, point) {
            Err(format!("could not set point"))
        }

        Ok(())
    }

    pub fn point(&self, point: usize) -> Result<TwoDimensionalPoint, String> {
        self.curve.point(point).ok_or(format!("could not get point"))
    }

    pub fn get_point(&self, t: f32) -> TwoDimensionalPoint {
        self.get_point(t)
    }
}

/// Represents an nth-order bezier curve.
#[derive(Clone, Debug)]
pub struct Curve<P: Point + Copy> {
    points: Vec<P>
}

impl<P: Point + Copy> Curve<P> {
    pub fn new() -> Self {
        Curve {
            points: Vec::new()
        }
    }

    /// Returns a point in this curve.
    pub fn point(&self, point: usize) -> Option<P> {
        self.points.get(point)
    }

    /// Adds a point to the bezier curve.
    pub fn add_point(&mut self, point: P) {
        self.points.push(point)
    }

    pub fn set_point(&mut self, i: usize, point: P) -> bool {
        if let Some(p) = self.points.get_mut(i) {
            *p = point;
            return true;
        }

        false
    }

    /// Returns an iterator that steps through the curve in `step`-sized segments.
    pub fn iter(&self, step: f32) -> Iter<P> {
        Iter::new(&self.curve, step)
    }

    /// Gets the order of this bezier curve. If the curve has less than two points,
    /// this will always return None.
    pub fn order(&self) -> Option<usize> {
        if self.points.len() < 2 {
            None
        } else {
            Some(self.points.len() - 1)
        }
    }

    /// Gets a single point on this bezier curve.
    pub fn get_point(&self, t: f32) -> P {
        let mut lines = Vec::with_capacity(self.points.len() - 1);

        for i in 0..self.points.len() - 1 {
            lines.push(Line(self.points[i], self.points[i + 1]));
        }

        // Interpolate between the ith line and the i + 1th line.
        // Replace the line at i with a new line based on these points.
        // Repeat until the amount of lines is equal to 1.

        // this is very clearly O(n^2)
        // however, in most use cases, n is probably gonna be 2 or 3
        while lines.len() > 1 {
            for i in 0..lines.len() - 1 {
                let a = lines[i].get_point(t);
                let b = lines[i + 1].get_point(t);

                lines[i] = Line(a, b);
            }

            lines.pop();
        }

        lines[0].get_point(t)
    }
}

pub struct Iter<'a, P: Point + Copy> {
    curve: &'a Curve<P>,
    cur: f32,
    step: f32
}

impl<'a, P: Point + Copy> Iter<P> {
    fn new(curve: &'a Curve<P>, step: f32) -> Self {
        Iter {
            curve,
            cur: 0.0,
            step
        }
    }
}

impl<P: Point + Copy> Iterator for Iter<P> {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > 1.0 {
            self.cur = 0.0;
            return None;
        }

        let res = if self.cur + self.step < 1.0 {
            self.curve.get_point(step);
        } else {
            self.curve.get_point(1.0);
        };

        self.cur += self.step;

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::curve::Curve;
    use crate::TwoDimensionalPoint;

    #[test]
    fn test_get_point() {
        let mut curve = Curve::<TwoDimensionalPoint>::new();
        curve.add_point(TwoDimensionalPoint::new(0.0, 0.0));
        curve.add_point(TwoDimensionalPoint::new(0.5, 0.5));
        curve.add_point(TwoDimensionalPoint::new(1.0, 0.0));

        let point = curve.get_point(0.5);
        assert_eq!(point.x(), 0.5);
        assert_eq!(point.y(), 0.25);
    }
}

pub struct Line<P: Point + Copy>(P, P);

impl<P: Point + Copy> Line<P> {
    fn get_point(&self, t: f32) -> P {
        self.0.lerp(self.1, t)
    }
}