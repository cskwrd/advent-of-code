fn main() {
    let filename = std::env::args().nth(1).unwrap(); // find a safer way to handle Option<>
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines_in_file = contents.lines().map(|l| l.chars().collect()).collect();

    let mut bit = 0;
    let (zeros, ones) = sort_bit_string_by_selected_bit(lines_in_file, bit);

    let mut oxygen_diagnostics = select_oxygen_diagnostics(&zeros, &ones);
    let mut co2_diagnostics = select_co2_diagnostics(&zeros, &ones);
    println!("co2_diagnostics.len: {}", co2_diagnostics.len());
    
    while oxygen_diagnostics.len() > 1 {
        bit += 1;
        
        // println!("bit: {}\nzero.len: {}\nones.len: {}\noxygen_diagnostics.len: {}", bit, zeros.len(), ones.len(), oxygen_diagnostics.len());

        let (oxygen_zeros, oxygen_ones) = sort_bit_string_by_selected_bit(oxygen_diagnostics, bit); // define new zeros and ones vectors because destrcturing assignments are still in beta

        oxygen_diagnostics = select_oxygen_diagnostics(&oxygen_zeros, &oxygen_ones);
        
        // println!("bit: {}\nzero.len: {}\nones.len: {}\noxygen_diagnostics.len: {}", bit, zeros.len(), ones.len(), oxygen_diagnostics.len());
    }

    bit = 0;
    while co2_diagnostics.len() > 1 {
        bit += 1;
        
        let (co2_zeros, co2_ones) = sort_bit_string_by_selected_bit(co2_diagnostics, bit); // define new zeros and ones vectors because destrcturing assignments are still in beta
        // println!("bit: {}\nzero.len: {}\nones.len: {}\nco2_diagnostics.len: {}", bit, co2_zeros.len(), co2_ones.len(), co2_diagnostics.len());

        co2_diagnostics = select_co2_diagnostics(&co2_zeros, &co2_ones);
        
        println!("bit: {}\nzero.len: {}\nones.len: {}\nco2_diagnostics.len: {}", bit, co2_zeros.len(), co2_ones.len(), co2_diagnostics.len());
    }
    println!("co2_diagnostics.len: {}", co2_diagnostics.len());

    let oxygen_rating_vector: String = oxygen_diagnostics.get(0).unwrap().into_iter().collect(); // have to convert vector back to string
    let oxygen_rating = isize::from_str_radix(oxygen_rating_vector.as_str(), 2).unwrap() as i32;

    let co2_rating_vector: String = co2_diagnostics.get(0).unwrap().into_iter().collect();
    let co2_rating = isize::from_str_radix(co2_rating_vector.as_str(), 2).unwrap() as i32;

    println!("{} * {} = {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);

    println!("main() - Complete!");
}

fn sort_bit_string_by_selected_bit(bit_strings: Vec<Vec<char>>, bit: usize) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut ones: Vec<Vec<char>> = Vec::new();
    let mut zeros: Vec<Vec<char>> = Vec::new();

    for bit_string in bit_strings {
        match bit_string.get(bit) {
            Some('0') => zeros.push(bit_string),
            Some('1') => ones.push(bit_string),
            _ => panic!("encountered invalid bit string!"),
        }
    }

    (zeros, ones)
}

fn select_oxygen_diagnostics(zeros: &Vec<Vec<char>>, ones: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    match zeros.len() > ones.len() {
        true => zeros.to_vec(), // use to_vec to copy instead of borrow the vec
        false => ones.to_vec(), // use to_vec to copy instead of borrow the vec
    }
}

fn select_co2_diagnostics(zeros: &Vec<Vec<char>>, ones: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    match zeros.len() > ones.len() {
        true => ones.to_vec(), // use to_vec to copy instead of borrow the vec
        false => zeros.to_vec(), // use to_vec to copy instead of borrow the vec
    }
}
