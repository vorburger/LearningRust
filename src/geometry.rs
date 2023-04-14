// https://google.github.io/comprehensive-rust/exercises/day-2/points-polygons.html

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

type Geo = i32;

use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    x: Geo,
    y: Geo,
}

impl Point {
    fn new(x: Geo, y: Geo) -> Self {
        Point { x, y }
    }
    fn magnitude(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
    fn dist(self, p2: &Point) -> f64 {
        let dx = p2.x - self.x;
        let dy = p2.y - self.y;
        ((dx * dx + dy * dy) as f64).sqrt()
    }
}

impl Add for Point {
    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    type Output = Point;
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    // fn iter(&self) -> std::slice::Iter<'_, Point> {
    fn iter(&self) -> impl Iterator<Item = &Point> {
        return self.points.iter();
    }

    fn left_most_point(&self) -> Option<Point> {
        let mut lmp: Option<Point> = None;
        for p in &self.points {
            match lmp {
                None => {
                    lmp = Some(*p);
                }
                Some(current_left_most) => {
                    if p.x < current_left_most.x {
                        lmp = Some(*p);
                    }
                }
            }
        }
        lmp
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Self {
        Circle { center, radius }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}
