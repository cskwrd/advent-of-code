use std::{collections::{LinkedList}, fmt::{Display, self, Debug}};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(config.input_file).expect("Something went wrong reading the file");
    let input = contents.lines().next().unwrap();

    let mut school_of_fish: LinkedList<LanternFish> = LinkedList::new();
    // let mut something_else: HashMap<i32, LanternFish> = HashMap::new();
    // for fish_timer in input.split(',') {
    //     let new_fish = LanternFish::new(fish_timer.parse().unwrap(), 0);
    //     if let Some(existing_fish) = something_else.get_mut(&new_fish.timer) {
    //         (*existing_fish).mutations += 1;
    //     } else {
    //         something_else.insert(new_fish.timer, new_fish);
    //         school_of_fish.push_back(new_fish.clone());
    //     }
    // }
    for fish_timer in input.split(',') {
        school_of_fish.push_back(LanternFish::new(fish_timer.parse().unwrap(), 0))
    }

    // println!("School: {:?}", school_of_fish);

    if input.len() <= 10 {
        println!("Initial state: [{}]", input);
    } else {
        println!("Initial state: [{} fish]", school_of_fish.len());
    }
    
    let days = config.days;
    let mut baby_sharks: LinkedList<LanternFish> = LinkedList::new();
    for i in 0..days {
        for fish in school_of_fish.iter_mut() {
            if let Some(baby_shark) = fish.advance_age() {
                baby_sharks.push_back(baby_shark);
            }
        }
        school_of_fish.append(&mut baby_sharks);
        println!("After {} days: {:?}", i+1, school_of_fish);
    }

    println!("{} fish are in class", school_of_fish.len());
}

#[derive(Clone, Copy)]
struct LanternFish {
    timer: i32,
    mutations: i32,
}

impl LanternFish {
    fn new(timer: i32, mutations: i32) -> LanternFish {
        LanternFish { timer, mutations }
    }

    fn advance_age(&mut self) -> Option<LanternFish> {
        self.timer -= 1;
        match self.timer {
            -1 => {
                self.timer = 6;
                Some(LanternFish::new(8, 0))
            },
            _ => None,
        }
    }
}

impl Display for LanternFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.timer)
    }
}

impl Debug for LanternFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.mutations + 1, self.timer)
    }
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
