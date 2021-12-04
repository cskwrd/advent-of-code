use std::str::Lines;

fn main() {
    let filename = std::env::args().nth(1).unwrap(); // find a safer way to handle Option<>
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let (oxygen_rating, co2_rating) = get_life_support_ratings(contents.lines());

    println!("{} * {} = {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating)
}

fn get_life_support_ratings(diagnostic_readings: Lines) -> (i32, i32) {
    let mut bit = 0;
    let mut zeros: Vec<Vec<char>> = Vec::new();
    let mut ones: Vec<Vec<char>> = Vec::new();

    for diagnostic in diagnostic_readings {
        if diagnostic.is_empty() {
            continue;
        }
        let diagnostic_data: Vec<char> = diagnostic.chars().collect(); // have to use a Vec<Vec<char>> go i can call get, bc nth consumes values in iterator
        match diagnostic_data.get(bit) {
            Some('0') => zeros.push(diagnostic_data),
            Some('1') => ones.push(diagnostic_data),
            _ => continue,
        }
    }

    let mut oxygen_diagnostics = match zeros.len() > ones.len() {
        true => zeros.to_vec(), // use to_vec to copy instead of borrow the vec
        false => ones.to_vec(), // use to_vec to copy instead of borrow the vec
    };
    let mut co2_diagnostics = match zeros.len() > ones.len() {
        true => ones.to_vec(), // use to_vec to copy instead of borrow the vec
        false => zeros.to_vec(), // use to_vec to copy instead of borrow the vec
    };
    
    while oxygen_diagnostics.len() > 1 {
        bit += 1;
        zeros.clear();
        ones.clear();
        
        println!("bit: {}\nzero.len: {}\nones.len: {}\noxygen_diagnostics.len: {}", bit, zeros.len(), ones.len(), oxygen_diagnostics.len());

        for diagnostic in oxygen_diagnostics {
            println!("{:?}", diagnostic);
            match diagnostic.get(bit) {
                Some('0') => zeros.push(diagnostic),
                Some('1') => ones.push(diagnostic),
                _ => continue,
            }
        }

        oxygen_diagnostics = match zeros.len() > ones.len() {
            true => zeros.to_vec(), // use to_vec to copy instead of borrow the vec
            false => ones.to_vec(), // use to_vec to copy instead of borrow the vec
        };
        
        println!("bit: {}\nzero.len: {}\nones.len: {}\noxygen_diagnostics.len: {}", bit, zeros.len(), ones.len(), oxygen_diagnostics.len());
    }
    println!("bit: {}\nzero.len: {}\nones.len: {}\noxygen_diagnostics.len: {}", bit, zeros.len(), ones.len(), oxygen_diagnostics.len());
    
    bit = 0;
    while co2_diagnostics.len() > 1 {
        bit += 1;
        zeros.clear();
        ones.clear();

        for diagnostic in co2_diagnostics {
            match diagnostic.get(bit) {
                Some('0') => zeros.push(diagnostic),
                Some('1') => ones.push(diagnostic),
                _ => continue,
            }
        }

        co2_diagnostics = match zeros.len() > ones.len() {
            true => ones.to_vec(), // use to_vec to copy instead of borrow the vec
            false => zeros.to_vec(), // use to_vec to copy instead of borrow the vec
        };
    }
    println!("bit: {}\nzero.len: {}\nones.len: {}\nco2_diagnostics.len: {}", bit, zeros.len(), ones.len(), co2_diagnostics.len());

    let oxygen_rating_vector: String = oxygen_diagnostics.get(0).unwrap().into_iter().collect(); // have to convert vector back to string
    let oxygen_rating = isize::from_str_radix(oxygen_rating_vector.as_str(), 2).unwrap() as i32;

    let co2_rating_vector: String = co2_diagnostics.get(0).unwrap().into_iter().collect();
    let co2_rating = isize::from_str_radix(co2_rating_vector.as_str(), 2).unwrap() as i32;

    (oxygen_rating, co2_rating)
}

// tried to use the folowing methods but i couldn't get the ownership correct... that will be a problem for another day

// fn sort_lines_by_selected_bit(lines: Lines, &bit: i32) -> (Vec<String>, Vec<String>) {
//     let mut zeros: Vec<String> = Vec::new();
//     let mut ones: Vec<String> = Vec::new();

//     for line in lines {
//         if line.is_empty() {
//             continue;
//         }
//         match line.chars().nth(bit) {
//             Some('0') => zeros.push(line),
//             Some('1') => zeros.push(line),
//             _ => continue,
//         }
//     }

//     (zeros, ones)
// }

// fn select_oxygen_diagnostics(zeros: &Vec<Chars>, ones: &Vec<Chars>) -> Vec<Chars> {
//     let mut diagnostics: Vec<Chars>;

//     if zeros.len() > ones.len() {
//         diagnostics = *zeros;
//     } else {
//         diagnostics = *ones;
//     }

//     diagnostics
// }

// fn select_co2_diagnostics(zeros: &Vec<Chars>, ones: &Vec<Chars>) -> Vec<Chars> {
//     let diagnostics: Vec<String>;
    
//     if zeros.len() > ones.len() {
//         diagnostics = *ones;
//     } else {
//         diagnostics = *zeros;
//     }

//     diagnostics
// }
