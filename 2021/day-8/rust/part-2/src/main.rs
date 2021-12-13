fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(config.input_file).expect("Something went wrong reading the file");
    let numbers_with_unique_display_segment_count: usize = contents.lines()
        .map(|l| l.split_once('|').unwrap().1.trim().split_ascii_whitespace().filter(|o| o.len() != 5 && o.len() != 6).count())
        .sum();
        
    println!("Number of numbers with unique segments: {}", numbers_with_unique_display_segment_count);
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
