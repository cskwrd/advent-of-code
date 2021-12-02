fn main() {
    let filename = std::env::args().nth(1).unwrap(); // find a safer way to handle Option<>

    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut position = 0;
    let mut depth =  0;
    let mut aim = 0;
    for line in contents.lines() {
        let mut instruction = line.split_ascii_whitespace();
        
        match instruction.next() {
            Some("forward") => {
                if let Some(magnitude) = instruction.next() {
                    match magnitude.parse::<i32>() {
                        Ok(value) => {
                            position += value;
                            depth += aim * value;
                        },
                        Err(_) => break, // should really handle this case some how so we don't print a partial answer, try/catch maybe?
                    };
                    
                }
                continue;
            },
            Some("down") => {
                if let Some(delta) = instruction.next() {
                    aim = match delta.parse::<i32>() {
                        Ok(value) => aim + value, // i wonder if there is a way to merge this match with the "up" match and just change the operation, like a case statement without a break in C#
                        Err(_) => break,
                    };
                }
                continue;
            },
            Some("up") => {
                if let Some(delta) = instruction.next() {
                    aim = match delta.parse::<i32>() {
                        Ok(value) => aim - value,
                        Err(_) => break,
                    };
                }
                continue;
            },
            _ => break, // _ instead of None is used to act as a wildcard
        }
    }

    println!("position: {}", position);
    println!("depth: {}", depth);
    println!("{} * {} = {}", position, depth, position * depth);
}
