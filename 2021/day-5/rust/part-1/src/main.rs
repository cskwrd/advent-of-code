use core::fmt;
use std::{fmt::Display, collections::HashMap};

fn main() {
    println!("\nbeginning hydrothermal analysis...");
    
    let filename = std::env::args().nth(1).unwrap(); // could make this safer using `if let`
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines_in_file = contents.lines();

    let mut sea_floor: HashMap<Point, i32> = HashMap::new();
    let mut num_points_of_interest = 0;
    for reading in lines_in_file {
        if let Some((segment_start, segment_end)) = reading.split_once(" -> ") {
            let start_point = parse_coordinates(segment_start).expect("unable to parse segment start");
            let end_point = parse_coordinates(segment_end).expect("unable to parse segment end");
            if interesting_segment(&start_point, &end_point) {
                let x: Point = make_positive_point(start_point.x, end_point.x);
                let y: Point = make_positive_point(start_point.y, end_point.y);
                for i in x.x..=x.y {
                    for j in y.x..=y.y {
                        let point_of_interest = Point { x: i, y: j };

                        // the following line throws a warning, but trying to fix it makes the contained code throw an error
                        // comparing to the example at https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert
                        // i think the problem comes from the `let` part
                        // if let point_of_interest_magnitude = sea_floor.entry(point_of_interest).or_insert(0) {
                        //     *point_of_interest_magnitude += 1;
                        //     if *point_of_interest_magnitude == 2 {
                        //         num_points_of_interest += 1;
                        //     }
                        // }

                        if let Some(point_of_interest_magnitude) = sea_floor.get_mut(&point_of_interest) {
                            *point_of_interest_magnitude += 1;
                            if *point_of_interest_magnitude == 2 {
                                // only count the point of interest once
                                num_points_of_interest += 1;
                            }
                        } else {
                            sea_floor.insert(point_of_interest, 1);
                        }
                    }
                }
            }
        }
    }

    println!("\nfound {} points of interest!!\n\nending hydrothermal analysis...\n", num_points_of_interest);
}

fn make_positive_point(first_coordinate: i32, second_coordinate: i32) -> Point {
    match first_coordinate <= second_coordinate {
        true => Point { x: first_coordinate, y: second_coordinate },
        false => Point { x: second_coordinate, y: first_coordinate },
    }
}

fn interesting_segment(start: &Point, end: &Point) -> bool { // &'s are needed so we don't take ownership from caller
    start.x == end.x || start.y == end.y
}

fn parse_coordinates(coordinates: &str) -> Option<Point> {
    if let Some((x_coordinate, y_coordinate)) = coordinates.split_once(',') {
        return Some(Point { x: x_coordinate.parse().expect("unable to parse x coordinate"), y: y_coordinate.parse().expect("unable to parse y coordinate") });
    }
    None
}

#[derive(PartialEq, Eq, Hash)] // this appears to "auto-implement" these functions or traits as they're called
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}
