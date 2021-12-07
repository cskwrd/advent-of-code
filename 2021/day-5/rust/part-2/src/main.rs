use core::fmt;
use std::{fmt::Display, collections::HashMap};

use num::Integer;

fn main() {
    println!("\nbeginning hydrothermal analysis...");
    
    let filename = std::env::args().nth(1).unwrap(); // could make this safer using `if let`
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let line_segments_in_file = contents.lines()
        .map(|l| {
            let (start, end) = l.split_once(" -> ").unwrap();
            let (start_x, start_y) = start.trim().split_once(',').unwrap();
            let (end_x, end_y) = end.trim().split_once(',').unwrap();
            LineSegment::new(Point { x: start_x.parse().unwrap(), y: start_y.parse::<i32>().unwrap() }, Point { x: end_x.parse().unwrap(), y: end_y.parse::<i32>().unwrap() })
        });
        // .filter(|ls| {
        //     is_horizontal(ls) || is_vertical(ls) || is_diagonal(ls)
        // });

    let mut sea_floor: HashMap<Point, i32> = HashMap::new();
    let mut num_points_of_interest = 0;
    for line_segment in line_segments_in_file {
        let mut is_interesting = false;
        if is_horizontal(&line_segment) {
            println!("considering the horizontal segment: {}", line_segment);
            is_interesting = true;
        } else if is_vertical(&line_segment) {
            println!("considering the vertical segment: {}", line_segment);
            is_interesting = true;
        } else if is_diagonal(&line_segment) {
            println!("considering the diagonal segment: {}", line_segment);
            is_interesting = true;
        }
        for point_of_interest in line_segment.get_points_on_segment() {
            if let Some(point_of_interest_magnitude) = sea_floor.get_mut(&point_of_interest) {
                *point_of_interest_magnitude += 1;
                if *point_of_interest_magnitude == 2 && is_interesting{
                    // only count the point of interest once
                    num_points_of_interest += 1;
                }
            } else {
                sea_floor.insert(point_of_interest, 1);
            }
        }
        println!("moving to next segment");
    }

    println!("\nfound {:?} points of interest!!\n\nending hydrothermal analysis...\n", num_points_of_interest);
}

fn is_horizontal(line_segment: &LineSegment) -> bool { // &'s are needed so we don't take ownership from caller
    line_segment.start.x == line_segment.end.x
}

fn is_vertical(line_segment: &LineSegment) -> bool { // &'s are needed so we don't take ownership from caller
    line_segment.start.y == line_segment.end.y
}

fn is_diagonal(line_segment: &LineSegment) -> bool { // &'s are needed so we don't take ownership from caller
// (line_segment.slope as f64).atan().to_degrees() == 45_f64 // this line doesn't seem to work, not exactly sure why, maybe my math is wrong...
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
}

impl LineSegment {
    fn new(start: Point, end: Point) -> LineSegment {
        let mut rise = end.y - start.y;
        let mut run = end.x - start.x;

        let gcd = rise.gcd(&run);

        // simplify slope fraction, so point calculation is easier. both rise/run will be == 1 if slope is == 1
        rise /= gcd;
        run /= gcd;

        let slope: i32;
        if run != 0 {
            slope = rise / run;
        } else {
            slope = 0;
        }

        LineSegment { start, end, rise, run, slope }
    }

    fn get_points_on_segment(&self) -> Vec<Point> {
        let mut x = self.start.x;
        let mut y = self.start.y;

        let mut points_on_segment: Vec<Point> = Vec::new();

        while x != self.end.x || y != self.end.y {
            let p = Point { x, y };
            println!("generated: {}", p);
            points_on_segment.push(p);
            x += self.run;
            y += self.rise;
        }

        points_on_segment.push(Point { x, y });

        points_on_segment
    }
}

impl Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LineSegment {{ ({},{}) -> ({},{}) }}", self.start.x, self.start.y, self.end.x, self.end.y)
    }
}
