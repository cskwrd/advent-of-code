use std::collections::VecDeque;

fn main() -> anyhow::Result<()> {
    // it is my understanding that include_str "opens" the file at compile time
    // this means that we check the file exists at compile time and don't need to handle an error here
    // it also means that the file's contents is compiled into the resulting binary
    // normally that wouldn't be ideal
    let input_lines = include_str!("../../../aoc-input.txt").lines();

    let mut stacks = Vec::<VecDeque<char>>::new();
        
    let mut stack = VecDeque::<char>::new();
    stack.push_front('S');
    stack.push_front('C');
    stack.push_front('V');
    stack.push_front('N');
    stacks.push(stack);

    stack = VecDeque::<char>::new();
    stack.push_front('Z');
    stack.push_front('M');
    stack.push_front('J');
    stack.push_front('H');
    stack.push_front('N');
    stack.push_front('S');
    stacks.push(stack);

    stack = VecDeque::<char>::new();
    stack.push_front('M');
    stack.push_front('C');
    stack.push_front('T');
    stack.push_front('G');
    stack.push_front('J');
    stack.push_front('N');
    stack.push_front('D');
    stacks.push(stack);

    stack = VecDeque::<char>::new();
    stack.push_front('T');
    stack.push_front('D');
    stack.push_front('F');
    stack.push_front('J');
    stack.push_front('W');
    stack.push_front('R');
    stack.push_front('M');
    stacks.push(stack);
        
    let mut stack = VecDeque::<char>::new();
    stack.push_front('P');
    stack.push_front('F');
    stack.push_front('H');
    stacks.push(stack);
        
    let mut stack = VecDeque::<char>::new();
    stack.push_front('C');
    stack.push_front('T');
    stack.push_front('Z');
    stack.push_front('H');
    stack.push_front('J');
    stacks.push(stack);

    stack = VecDeque::<char>::new();
    stack.push_front('D');
    stack.push_front('P');
    stack.push_front('R');
    stack.push_front('Q');
    stack.push_front('F');
    stack.push_front('S');
    stack.push_front('L');
    stack.push_front('Z');
    stacks.push(stack);

    stack = VecDeque::<char>::new();
    stack.push_front('C');
    stack.push_front('S');
    stack.push_front('L');
    stack.push_front('H');
    stack.push_front('D');
    stack.push_front('F');
    stack.push_front('P');
    stack.push_front('W');
    stacks.push(stack);

    stack = VecDeque::<char>::new();
    stack.push_front('D');
    stack.push_front('S');
    stack.push_front('M');
    stack.push_front('P');
    stack.push_front('F');
    stack.push_front('N');
    stack.push_front('G');
    stack.push_front('Z');
    stacks.push(stack);

    println!("Top of stacks: '{}'", solve(stacks, input_lines));
    
    Ok(())
}

fn solve(mut stacks: Vec<VecDeque<char>>, input_lines: std::str::Lines) -> String {
    for line in input_lines {
        let tokens = line.split(' ').collect::<Vec<&str>>();
        let num_crates = tokens.get(1).expect("cannot read number of crates to move").parse::<usize>().expect("cannot parse number of crates to move");
        let src_stack = tokens.get(3).expect("cannot read source stack index").parse::<usize>().expect("cannot parse source stack index") - 1;
        let dst_stack = tokens.get(5).expect("cannot read destination stack index").parse::<usize>().expect("cannot parse destination stack index") - 1;
        
        let mut crane = VecDeque::<char>::new();
        let mut stack = stacks.get_mut(src_stack).expect("cannot find source stack");
        while crane.len() < num_crates {
            if stack.is_empty() {
                panic!("source stack is empty");
            }
            crane.push_front(stack.pop_front().expect("cannot pull another crate from stack"));
        }
        stack = stacks.get_mut(dst_stack).expect("cannot find destination stack");
        while !crane.is_empty() {
            stack.push_front(crane.pop_back().expect("crane could not release crate"));
        }
    }
    let mut top_crates = String::new();
    for mut s in stacks {
        top_crates.push_str(s.pop_front().expect("cannot read top crate on stack").to_string().as_str());
    }
    top_crates
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_solver() {
        // Please note, that private functions can be tested too!
        let input_lines = include_str!("../../../aoc-example-1.txt").lines();

        let mut stacks = Vec::<VecDeque<char>>::new();
        
        let mut stack = VecDeque::<char>::new();
        stack.push_front('Z');
        stack.push_front('N');
        stacks.push(stack);

        stack = VecDeque::<char>::new();
        stack.push_front('M');
        stack.push_front('C');
        stack.push_front('D');
        stacks.push(stack);

        stack = VecDeque::<char>::new();
        stack.push_front('P');
        stacks.push(stack);

        assert_eq!(solve(stacks, input_lines), "CMZ");
    }
}
