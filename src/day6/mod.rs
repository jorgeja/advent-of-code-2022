use std::collections::HashSet;

fn parse_example(input: &str) -> Vec<(u32, String)> {
    input.lines().filter_map(|line| {
        let mut elems = line.split(" ");
        let expected_value = elems.next()?.parse().ok()?;
        let s = elems.next()?;
        Some((expected_value, s.into()))
    }).collect()
}

fn find_start_of_message(input: &str, window_size: usize) -> u32 {    
    let chars = input.chars().collect::<Vec<char>>();
    let mut set = HashSet::new();
    for (i, window) in chars.as_slice().windows(window_size).enumerate() {      
        //eprint!("{i}: {:?}", window);
        window.iter().for_each(|c| {set.insert(*c);});
        if set.len() == window.len() {
            //eprintln!(" unique: {:?}", set);
            return (i + window_size) as u32;
        } else {
            //eprintln!("");
        }
        set.clear();
    }
    return 0;
}
#[cfg(test)]
mod day6 {
    use super::*;
    #[test]
    fn solve_example() {
        let input = include_str!("example.txt"); 
        let verification = parse_example(input);
        for (expected_value, s) in verification {
            let result = find_start_of_message(&s, 4);
            eprintln!("result {}", result);
            assert_eq!(result, expected_value);
        }        
    }

    #[test]
    fn solve_input_1() {
        let input = include_str!("input.txt"); 
        let result = find_start_of_message(input, 4);
        eprintln!("Result: {}", result);
    }

    #[test]
    fn solve_example_2() {
        let input = include_str!("example.txt"); 
        //let result = parse_and_solve_part2(input);
        //assert_eq!(result, 4);
    }
    
    #[test]
    fn solve_input_2() {
        let input = include_str!("input.txt"); 
        let result = find_start_of_message(input, 14);
        eprintln!("Result: {}", result);
    }
}
