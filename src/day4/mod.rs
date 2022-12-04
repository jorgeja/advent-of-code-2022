use std::ops::Range;


fn parse_and_solve(input: &str) -> u32 {    
    input.lines().map(parse_line).sum()
}

fn parse_line(line: &str) -> u32 {
    let mut pair = line.split(",").filter_map(into_range);
    let left = pair.next().expect("Expecting valid input");
    let right = pair.next().expect("Expecting valid input");
    if (left.contains(&right.start) && left.contains(&(right.end-1))) || (right.contains(&left.start) && right.contains(&(left.end-1))) {
        //eprintln!("left: {:?}, right: {:?}, CONTAINED", left, right);
        1 
    } else {
        //eprintln!("left: {:?}, right: {:?}, NOT CONTAINED", left, right);
        0
    }
}

fn into_range(r: &str) -> Option<Range<u32>> {
    let mut bounds = r.split("-");
    let start: u32 = bounds.next()?.parse().ok()?;
    let end: u32 = bounds.next()?.parse().ok()?;
    Some(Range{start, end: end+1})
}

fn parse_line_2(line: &str) -> u32 {
    let mut pair = line.split(",").filter_map(into_range);
    let left = pair.next().expect("Expecting valid input");
    let right = pair.next().expect("Expecting valid input");
    if left.contains(&right.start) || left.contains(&(right.end-1)) || (right.contains(&left.start) || right.contains(&(left.end-1))) {
        //eprintln!("left: {:?}, right: {:?}, CONTAINED", left, right);
        1 
    } else {
        //eprintln!("left: {:?}, right: {:?}, NOT CONTAINED", left, right);
        0
    }
}

fn parse_and_solve_part2(input: &str) -> u32 {    
    input.lines().map(parse_line_2).sum()
} 

#[cfg(test)]
mod day4 {
    use super::*;
    #[test]
    fn parse_example() {
        let input = include_str!("example.txt"); 
        let result = parse_and_solve(input);
        eprintln!("Result: {}", result);
        assert_eq!(result, 2);
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
        assert_eq!(result, 4);
    }
    
    #[test]
    fn solve_input_2() {
        let input = include_str!("input.txt"); 
        let result = parse_and_solve_part2(input);
        eprintln!("Input2: {}", result)
    }
}
