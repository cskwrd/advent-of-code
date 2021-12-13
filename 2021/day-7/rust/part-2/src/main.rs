use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(config.input_file).expect("Something went wrong reading the file");
    let crab_positions = parse_input(contents.lines());
    
    let average_crab_position = (crab_positions.iter().sum::<i32>() as f32 / crab_positions.len() as f32).round() as i32;
    dbg!(average_crab_position);

    let mut lowest_fuel_consumption = calculate_fuel_consumption(crab_positions.as_slice(), average_crab_position);
    let mut target_position = average_crab_position;
    let mut consumption_calculations: HashMap<i32, i32> = HashMap::new(); // store calculations so we don't need to recalc them over and over.
    consumption_calculations.insert(average_crab_position, lowest_fuel_consumption);
    let min_fuel: i32 = loop {
        let target_position_plus_one = target_position + 1;
        let target_position_minus_one = target_position - 1;

        let fuel_consumption_approaching_positive_inf = *consumption_calculations.entry(target_position_plus_one).or_insert(calculate_fuel_consumption(crab_positions.as_slice(), target_position_plus_one));
        let fuel_consumption_approaching_negative_inf = *consumption_calculations.entry(target_position_minus_one).or_insert(calculate_fuel_consumption(crab_positions.as_slice(), target_position_minus_one));

        if fuel_consumption_approaching_positive_inf < lowest_fuel_consumption {
            lowest_fuel_consumption = fuel_consumption_approaching_positive_inf;
            target_position = target_position_plus_one;
        } else if fuel_consumption_approaching_negative_inf < lowest_fuel_consumption {
            lowest_fuel_consumption = fuel_consumption_approaching_negative_inf;
            target_position = target_position_minus_one;
        } else {
            break lowest_fuel_consumption;
        }
    };

    println!("Min fuel: {}", min_fuel);
}

fn calculate_fuel_consumption(positions: &[i32], target_position: i32) -> i32 {
    positions.iter().map(|p| sorta_factorial(i32::abs(target_position - p))).sum()
}

fn sorta_factorial(n: i32) -> i32 {
    (1..=n).sum()
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
