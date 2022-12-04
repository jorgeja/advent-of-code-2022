use std::collections::HashSet;

fn parse_and_solve(input: &str) -> u32 {    
    input.lines().map(|line| {
        find_duplicate_item_priority(line)
    }).sum()
}

fn find_duplicate_item_priority(line: &str) -> u32 {
    let line_length = line.len();
    let compartment_1 = line[0..(line_length/2)].chars().collect::<HashSet<char>>();
    //eprintln!("Comp1: {:?}", compartment_1);
    let compartment_2 = line[(line_length/2)..].chars().collect::<HashSet<char>>();
    //eprintln!("Comp1: {:?}", compartment_2);
    let in_both = compartment_1.intersection(&compartment_2);
    //eprintln!("in both: {:?}", in_both);
    in_both.map(priority_of_char).sum()    
}

fn priority_of_char(c: &char) -> u32 {
    //eprint!("Duplicate char: {} -> ", c);
    let v = match c {
        'a'..='z' => *c as u32 - b'a' as u32 + 1,
        'A'..='Z' => *c as u32 - b'A' as u32 + 27,
        _ => 0
    };
    //eprint!("{}\n", v);
    v
}

fn find_duplicate_item_from_group(group: &[&str]) -> u32 {
    let one = group[0].chars().collect::<HashSet<char>>();
    let two = group[1].chars().collect::<HashSet<char>>();
    let one_two = one.intersection(&two).copied().collect::<HashSet<char>>();
    let three = group[2].chars().collect::<HashSet<char>>();
    one_two.intersection(&three).map(priority_of_char).sum()    
}

fn parse_and_solve_part2(input: &str) -> u32 {    
    let lines = input.lines().collect::<Vec<&str>>(); 
    lines.as_slice().chunks(3).map(find_duplicate_item_from_group).sum()
} 

#[cfg(test)]
mod day3 {
    use super::*;
    #[test]
    fn parse_example() {
        let input = include_str!("example.txt"); 
        let result = parse_and_solve(input);
        eprintln!("Result: {}", result);
        assert_eq!(result, 157);
    }

    #[test]
    fn solve_input_1() {
        let input = include_str!("input.txt"); 
        let result = parse_and_solve(input);
        eprintln!("Result: {}", result);
    }

    #[test]
    fn solve_example_2() {
        let input = include_str!("example.txt"); 
        let result = parse_and_solve_part2(input);
        assert_eq!(result, 70);
    }
    
    #[test]
    fn solve_input_2() {
        let input = include_str!("input.txt"); 
        let result = parse_and_solve_part2(input);
        eprintln!("Input2: {}", result)
    }
}
