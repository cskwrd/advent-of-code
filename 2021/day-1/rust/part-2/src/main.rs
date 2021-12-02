use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect(); //this returns a Vec<&str>, I wonder if it iterates the entire arrat like IEnumerable.ToArray()
    let mut depths: Vec<i32> = Vec::new();

    // a more efficient approach would be to take depths from the file 4 at a time and compare the two sets of 3 as you go

    for i in 0..lines.len()-2 {
        let depth1: i32 = lines[i].parse().unwrap();
        let depth2: i32 = lines[i+1].parse().unwrap();
        let depth3: i32 = lines[i+2].parse().unwrap();
        depths.push(depth1 + depth2 + depth3);
    }
    let mut previous_depth = -1;
    let mut positive_measurements = -1;
    for depth in depths {
        if depth > previous_depth {
            positive_measurements += 1;
        }
        previous_depth = depth;
    }

    println!("In this example, there are '{}' measurements that are larger than the previous measurement.", positive_measurements);
}
