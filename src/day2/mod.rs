
fn parse_and_solve(input: &str) -> i32 {    
    input.lines().map(|line| {
        let mut elements = line.split(" ");
        let left = elements.next().unwrap();
        let right = elements.next().unwrap();
        solve_single_game(left, right)
    }).sum()
}

fn solve_single_game(left: &str, right: &str) -> i32 {    
    match left {
        "A" => match right {
            "X" => 1 + 3,
            "Y" => 2 + 6,
            "Z" => 3 + 0,
            _ => panic!("bad input {} {}", left, right),
        },
        "B" => match right {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => panic!("bad input {} {}", left, right),
        },
        "C" => match right {
            "X" => 1 + 6,
            "Y" => 2 + 0,
            "Z" => 3 + 3,
            _ => panic!("bad input {} {}", left, right),
        },
        _ => panic!("bad input {} {}", left, right),
    }
}

fn parse_and_solve_part2(input: &str) -> i32 {    
    input.lines().map(|line| {
        let mut elements = line.split(" ");
        let left = elements.next().unwrap();
        let right = elements.next().unwrap();
        solve_single_game_part2(left, right)
    }).sum()
}

fn solve_single_game_part2(left: &str, right: &str) -> i32 {    
    match left {
        "A" => match right {
            "X" => 3 + 0,
            "Y" => 1 + 3,
            "Z" => 2 + 6,
            _ => panic!("bad input {} {}", left, right),
        },
        "B" => match right {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => panic!("bad input {} {}", left, right),
        },
        "C" => match right {
            "X" => 2 + 0,
            "Y" => 3 + 3,
            "Z" => 1 + 6,
            _ => panic!("bad input {} {}", left, right),
        },
        _ => panic!("bad input {} {}", left, right),
    }
}

#[cfg(test)]
mod day2 {
    use super::*;
    #[test]
    fn parse_example() {
        let input = include_str!("example.txt"); 
        let result = parse_and_solve(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn solve_input_1() {
        let input = include_str!("input.txt"); 
        let result = parse_and_solve(input);
        eprintln!("Input1: {}", result)
    }

    #[test]
    fn solve_example_2() {
        let input = include_str!("example.txt"); 
        let result = parse_and_solve_part2(input);
        assert_eq!(result, 12);
    }
    
    #[test]
    fn solve_input_2() {
        let input = include_str!("input.txt"); 
        let result = parse_and_solve_part2(input);
        eprintln!("Input2: {}", result)
    }
}
