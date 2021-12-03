fn main() {
    let filename = std::env::args().nth(1).unwrap(); // find a safer way to handle Option<>
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut bit_counter = [[0; 2]; 12];
    let mut gamma = ['0'; 12];
    let mut epsilon = ['0'; 12];
    for line in contents.lines() {
        let diagnostics = line.chars();

        for (i, diagnostic) in diagnostics.enumerate() {
            match diagnostic {
                '0' => {
                    bit_counter[i][0] += 1;
                },
                '1' => {
                    bit_counter[i][1] += 1;
                },
                _ => break,
            }
            if bit_counter[i][0] > bit_counter[i][1] {
                gamma[i] = '0';
                epsilon[i] = '1';
            } else {
                gamma[i] = '1';
                epsilon[i] = '0';
            }
        }
    }

    let gamma_rate = isize::from_str_radix(gamma.iter().collect::<String>().as_str(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(epsilon.iter().collect::<String>().as_str(), 2).unwrap();
    println!("{} * {} = {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}
