fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(config.input_file).expect("Something went wrong reading the file");
    let mut crab_positions = parse_input(contents.lines());
    crab_positions.sort();
    let middle_crab = crab_positions[crab_positions.len() / 2];

    let min_fuel: i32 = crab_positions.iter().map(|p| i32::abs(middle_crab - p)).sum();

    println!("Min fuel: {}", min_fuel);
}

fn parse_input(input: std::str::Lines) -> Vec<i32> {
    input.flat_map(|l| l.split(','))
        .filter_map(|v| v.parse().ok())
        .collect::<Vec<i32>>()
}

struct Config {
    input_file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("need more args");
        }

        let input_file = args[1].clone();

        Ok(Config { input_file })
    }
}
