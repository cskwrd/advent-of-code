use core::fmt;
use std::{fmt::Display, collections::HashMap, collections::HashSet};

use num::Integer;

fn main() {
    println!("\nbeginning hydrothermal analysis...");
    
    let filename = std::env::args().nth(1).unwrap(); // could make this safer using `if let`
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let endpoints_in_file = contents.lines()
        .map(|l| {
            let (start, end) = l.split_once(" -> ").unwrap();
            let (start_x, start_y) = start.trim().split_once(',').unwrap();
            let (end_x, end_y) = end.trim().split_once(',').unwrap();
            LineSegment::new(Point { x: start_x.parse().unwrap(), y: start_y.parse::<i32>().unwrap() }, Point { x: end_x.parse().unwrap(), y: end_y.parse::<i32>().unwrap() })
        })
        .filter(|ls| {
            is_horizontal(ls) || is_vertical(ls) || is_diagonal(ls)
        });

    for line_segment in endpoints_in_file {
        println!("{}", line_segment);
    }

    // println!("\nfound {:?} points of interest!!\n\nending hydrothermal analysis...\n", num_points_of_interest);
}

fn is_horizontal(line_segment: &LineSegment) -> bool { // &'s are needed so we don't take ownership from caller
    line_segment.start.x == line_segment.end.x
}

fn is_vertical(line_segment: &LineSegment) -> bool { // &'s are needed so we don't take ownership from caller
    line_segment.start.y == line_segment.end.y
}

fn is_diagonal(line_segment: &LineSegment) -> bool { // &'s are needed so we don't take ownership from caller
    line_segment.slope.abs() == 1
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)] // this appears to "auto-implement" these functions or traits as they're called
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

struct LineSegment {
    start: Point,
    end: Point,
    rise: i32,
    run: i32,
    slope: i32,
    b: i32,
}

impl LineSegment {
    fn new(start: Point, end: Point) -> LineSegment {
        let mut rise = end.y - start.y;
        let mut run = end.x - start.x;

        let gcd = rise.gcd(&run);

        rise /= gcd;
        run /= gcd;

        let slope: i32;
        if run != 0 {
            slope = rise / run;
        } else {
            slope = 0;
        }

        let b = start.y - (slope * start.x);

        LineSegment { start, end, rise, run, slope, b }
    }
}

impl Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LineSegment {{ ({},{}) -> ({},{}) }}", self.start.x, self.start.y, self.end.x, self.end.y)
    }
}
