use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter::FromIterator;
#[derive(Debug)]
enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise,
}
#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct ConvexHull<'a> {
    points: &'a mut Vec<Point>,
}

fn orientation(p: &Point, q: &Point, r: &Point) -> Orientation {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val > 0. {
        Orientation::Clockwise
    } else if val < 0. {
        Orientation::CounterClockwise
    } else {
        Orientation::Colinear
    }
}

fn dist_sq(p1: &Point, p2: &Point) -> f64 {
    (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
}

fn sort_by_y(p1: &Point, p2: &Point) -> Ordering {
    if p1.y < p2.y {
        Ordering::Less
    } else if p1.y > p2.y {
        Ordering::Greater
    } else {
        if p1.x < p2.x {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn check_angle(p0: &Point, p1: &Point, p2: &Point) -> Option<Orientation> {
    match orientation(p0, p1, p2) {
        Orientation::CounterClockwise => None,
        o => Some(o),
    }
}

impl<'a> ConvexHull<'a> {
    fn new(points: &mut Vec<Point>) -> ConvexHull {
        assert!(points.len() >= 3);
        ConvexHull { points: points }
    }

    fn sort_by_orientation(&mut self, p0: &Point) {
        self.points.sort_by(|p1: &Point, p2: &Point| -> Ordering {
            match orientation(&p0, p1, p2) {
                Orientation::Clockwise => Ordering::Greater,
                Orientation::CounterClockwise => Ordering::Less,
                Orientation::Colinear => {
                    let d1 = dist_sq(&p0, p1);
                    let d2 = dist_sq(&p0, p2);
                    if d2 >= d1 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            }
        });
    }

    fn convex_hull(&mut self) {
        self.points.sort_by(sort_by_y);
        let p0 = self.points.remove(0);
        self.sort_by_orientation(&p0);
        let mut result = vec![&p0];
        let mut points = VecDeque::from_iter(self.points.iter());
        let p1 = points.pop_front().unwrap();
        result.push(p1);
        let p2 = points.pop_front().unwrap();
        result.push(p2);
        for p in points {
            let mut result_len = result.len();
            while let Some(_) = check_angle(result[result_len - 2], result[result_len - 1], p) {
                result.pop();
                result_len -= 1;
            }
            result.push(p);
        }
        println!("{:?}", result);
    }
}

fn main() {
    let mut points = vec![
        Point { x: 0., y: 3. },
        Point { x: 1., y: 1. },
        Point { x: 2., y: 2. },
        Point { x: 4., y: 4. },
        Point { x: 0., y: 0. },
        Point { x: 1., y: 2. },
        Point { x: 3., y: 1. },
        Point { x: 3., y: 3. },
    ];
    let mut conv = ConvexHull::new(&mut points);
    conv.convex_hull();
}
