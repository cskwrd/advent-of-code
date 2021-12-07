use std::{collections::LinkedList, fmt::{Display, self, Debug}};

fn main() {
    let filename = std::env::args().nth(1).unwrap(); // could make this safer using `if let`
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let input = contents.lines().next().unwrap();

    let mut school_of_fish: LinkedList<LanternFish> = LinkedList::new();
    for fish_timer in input.split(',') {
        school_of_fish.push_back(LanternFish::new(fish_timer.parse().unwrap()))
    }

    if input.len() <= 10 {
        println!("Initial state: [{}]", input);
    } else {
        println!("Initial state: [{} fish]", school_of_fish.len());
    }
    
    let days = std::env::args().nth(2).unwrap().parse::<i32>().unwrap();
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

struct LanternFish {
    timer: i32,
}

impl LanternFish {
    fn new(timer: i32) -> LanternFish {
        LanternFish { timer }
    }

    fn advance_age(&mut self) -> Option<LanternFish> {
        self.timer -= 1;
        match self.timer {
            -1 => {
                self.timer = 6;
                Some(LanternFish::new(8))
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
        write!(f, "{}", self.timer)
    }
}
