use std::collections::{HashSet, VecDeque};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error occurred: {}", err);
        std::process::exit(1);
    });

    let contents = std::fs::read_to_string(config.input_file).expect("Something went wrong reading the file");
    // let contents = std::fs::read_to_string("..\\..\\aoc-input.small.txt").expect("Something went wrong reading the file");
    let entries = contents.lines()
        .map(|l| LogEntry::new(l));

    let mut output = 0;
    for mut entry in entries {
        // dbg!(&entry);
        dbg!(&output);
        let mut display = SevenSegmentDisplay::new();
        while let Some(signal) = entry.signal_patterns.pop_front() {
            if !display.encode(&signal) {
                entry.signal_patterns.push_back(signal);
            }
        }

        let mut display_output = String::from("");
        for display_signal in entry.output_display {
            display_output += display.decode(display_signal.chars().collect());
        }
        output += display_output.parse::<i32>().unwrap();
    }

    println!("{}", output);
}

#[derive(Debug)]
struct SevenSegmentDisplay {
    zero: HashSet<char>,
    one: HashSet<char>,
    two: HashSet<char>,
    three: HashSet<char>,
    four: HashSet<char>,
    five: HashSet<char>,
    six: HashSet<char>,
    seven: HashSet<char>,
    eight: HashSet<char>,
    nine: HashSet<char>,
}

impl SevenSegmentDisplay {
    fn new() -> SevenSegmentDisplay {
        SevenSegmentDisplay {
            zero: HashSet::new(),
            one: HashSet::new(),
            two: HashSet::new(),
            three: HashSet::new(),
            four: HashSet::new(),
            five: HashSet::new(),
            six: HashSet::new(),
            seven: HashSet::new(),
            eight: HashSet::new(),
            nine: HashSet::new(),
        }
    }
    
    fn encode(&mut self, signal: &HashSet<char>) -> bool {
        // dbg!(&self);
        // dbg!(&signal);
        let signal_length = signal.len();
        let mut encoding_successful = false;
        if signal_length == 2 {
            // showing 1 on display
            self.one = signal.clone();
            encoding_successful = true;
        } else if signal_length == 3 {
            // showing 7 on display
            self.seven = signal.clone();
            encoding_successful = true;
        } else if signal_length == 4 {
            // showing 4 on display
            self.four = signal.clone();
            encoding_successful = true;
        } else if signal_length == 7 {
            // showing 8 on display
            self.eight = signal.clone();
            encoding_successful = true;
        } else if signal_length == 5 {
            // showing 2, 3, or 5 on display
            if self.nine.difference(signal).count() == 1 && self.seven.len() > 0 && self.seven.difference(signal).count() == 0 {
                // showing 3 on display
                self.three = signal.clone();
                encoding_successful = true;
            } else if self.six.difference(signal).count() == 1 {
                // showing 5 on display
                self.five = signal.clone();
                encoding_successful = true;
            } else if self.three.len() > 0 && self.five.len() > 0 {
                // showing 2 on display
                self.two = signal.clone();
                encoding_successful = true;
            }
        } else if signal_length == 6 {
            // showing 0, 6, or 9 on display
            if self.four.len() > 0 && self.four.difference(signal).count() == 0 && self.eight.difference(signal).count() == 1 {
                // showing 9 on display
                self.nine = signal.clone();
                encoding_successful = true;
            } else if self.seven.difference(signal).count() == 1 {
                // showing 6 on display
                self.six = signal.clone();
                encoding_successful = true;
            } else if self.nine.len() > 0 && self.six.len() > 0 {
                // showing 0 on display
                self.zero = signal.clone();
                encoding_successful = true;
            }
        } else {
            panic!("not a valid signal");
        }
        encoding_successful
    }

    fn decode(&self, display_signal: HashSet<char>) -> &str {
        if self.zero.symmetric_difference(&display_signal).count() == 0 {
            return "0";
        } else if self.one.symmetric_difference(&display_signal).count() == 0 {
            return "1";
        } else if self.two.symmetric_difference(&display_signal).count() == 0 {
            return "2";
        } else if self.three.symmetric_difference(&display_signal).count() == 0 {
            return "3";
        } else if self.four.symmetric_difference(&display_signal).count() == 0 {
            return "4";
        } else if self.five.symmetric_difference(&display_signal).count() == 0 {
            return "5";
        } else if self.six.symmetric_difference(&display_signal).count() == 0 {
            return "6";
        } else if self.seven.symmetric_difference(&display_signal).count() == 0 {
            return "7";
        } else if self.eight.symmetric_difference(&display_signal).count() == 0 {
            return "8";
        } else if self.nine.symmetric_difference(&display_signal).count() == 0 {
            return "9";
        }
        panic!("bad display_output signal");
    }
}

#[derive(Debug)]
struct LogEntry {
    signal_patterns: VecDeque<HashSet<char>>,
    output_display: Vec<String>,
}

impl LogEntry {
    fn new(log_entry: &str) -> LogEntry {
        let mut signal_patterns: VecDeque<HashSet<char>> = VecDeque::new();
        let mut display_output: Vec<String> = Vec::new();

        let mut signals = log_entry.split_ascii_whitespace();
        while let Some(signal) = signals.next() {
            if signal == "|" {
                break;
            }
            signal_patterns.push_back(signal.chars().collect());
        }

        while let Some(output_character) = signals.next() {
            display_output.push(output_character.to_string());
        }

        LogEntry { signal_patterns: signal_patterns, output_display: display_output }
    }
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
