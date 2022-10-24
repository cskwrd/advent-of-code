use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(config.input_file).expect("Something went wrong reading the file");
    let input = contents.lines().next().unwrap();

    let mut school_of_fish_hash_map: HashMap<i32, i64> = HashMap::new();
    for fish_timer in input.split(',') {
        let timer = fish_timer.parse().unwrap();
        if let Some(bucket) = school_of_fish_hash_map.get(&timer) {
            school_of_fish_hash_map.insert(timer, bucket+1);
        } else {
            school_of_fish_hash_map.insert(timer, 1);
        }
    }
    println!("School: {:?}", school_of_fish_hash_map);
    
    let days = config.days;
    for i in 0..days {
        println!("Day: {}", i + 1);
        let mut new_school_of_fish_hash_map: HashMap<i32, i64> = HashMap::new();
        for internal_timer in (0..9).rev() {
            if let Some(existing_fish) = school_of_fish_hash_map.get(&internal_timer) {
                let (next_class, overflow) = get_next_class(&internal_timer);
                if overflow {
                    if new_school_of_fish_hash_map.contains_key(&8) == false {
                        new_school_of_fish_hash_map.insert(8, existing_fish.clone());
                    } else {
                        let new_class_of_fish = new_school_of_fish_hash_map.get(&8).unwrap();
                        let updated_class = existing_fish + new_class_of_fish;
                        new_school_of_fish_hash_map.insert(8, updated_class);
                    }
                }
                if new_school_of_fish_hash_map.contains_key(&next_class) == false {
                    new_school_of_fish_hash_map.insert(next_class, existing_fish.clone());
                } else {
                    let new_class_of_fish = new_school_of_fish_hash_map.get(&next_class).unwrap();
                    let updated_class = existing_fish + new_class_of_fish;
                    new_school_of_fish_hash_map.insert(next_class, updated_class);
                }
            }
        }
        school_of_fish_hash_map = new_school_of_fish_hash_map;
    }

    let mut num_fish = 0;
    for day in 0..9 {
        if let Some(fish) = school_of_fish_hash_map.get(&day) {
            num_fish += fish;
        }
    }
    // println!("School: {:?}", school_of_fish_hash_map);
    println!("Total number: {}", num_fish);
}

fn get_next_class(current_class: &i32) -> (i32, bool) {
    let next_class = current_class - 1;
    if next_class < 0 {
        return (6, true)
    }
    (next_class, false)
}

struct Config {
    input_file: String,
    days: i32,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("need more args");
        }

        let input_file = args[1].clone();
        let days_str = args[2].clone();

        let days: i32 = days_str.to_string().parse().unwrap();

        Ok(Config { input_file, days })
    }
}
