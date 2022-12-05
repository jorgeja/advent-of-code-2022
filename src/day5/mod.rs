
type Stacks = Vec<Vec<char>>;
type Operations = Vec<(usize, usize, usize)>;

fn parse(input: &str) -> (Stacks, Operations) {    
    let mut lines = input.lines();
    let mut stacks = vec![vec![]];
    for line in lines.by_ref() {
        if line.len() == 0 {
            break;
        }

        for (i, char) in line.chars().enumerate() {
            match (i % 4, char) {
                (1, 'A'..='Z') => {
                    let index = i / 4;
                    init_stacks_to_index(&mut stacks, index);
                    stacks[index].push(char);
                },
                _ => {}, 
            }
        }       
    }

    for stack in stacks.iter_mut() {
        stack.reverse();        
    }

    let operations = lines.filter_map(|line| {
        let mut params = line.split(" ");
        let amount: usize = params.nth(1)?.parse().ok()?;
        let from: usize = params.nth(1)?.parse().ok()?;
        let to: usize = params.nth(1)?.parse().ok()?;
        Some((amount, from-1, to-1))
    }).collect::<Operations>();



    (stacks, operations)    
}

fn result(stacks: &Stacks) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect::<String>()
}

fn move_crates(stacks: &mut Stacks, amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        let _crate = stacks[from].pop().unwrap();
        stacks[to].push(_crate);
    }
}

fn move_crates_2(stacks: &mut Stacks, amount: usize, from: usize, to: usize) {
    let mut temp = vec![];
    for _ in 0..amount {
        let _crate = stacks[from].pop().unwrap();
        temp.push(_crate);        
    }

    for _crate in temp.into_iter().rev() {
        stacks[to].push(_crate);
    }
}

fn init_stacks_to_index(stacks: &mut Vec<Vec<char>>, index: usize) {
    if index+1 > stacks.len() {
        for _ in stacks.len()..index+1 {
            stacks.push(Vec::default());
        }
    }
}

fn parse_and_solve_part2(input: &str) -> u32 {    
    0
} 

#[cfg(test)]
mod day4 {
    use super::*;
    #[test]
    fn parse_example() {
        let input = include_str!("example.txt"); 
        let (mut stacks, operations) = parse(input);
        operations.iter().for_each(|(amount, from, to)| {
            move_crates(&mut stacks, *amount, *from, *to);
        });
        let result = result(&stacks);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn solve_input_1() {
        let input = include_str!("input.txt"); 
        let (mut stacks, operations) = parse(input);
        operations.iter().for_each(|(amount, from, to)| {
            move_crates(&mut stacks, *amount, *from, *to);
        });
        let result = result(&stacks);
        eprintln!("Result1: {}", result);
    }

    #[test]
    fn solve_example_2() {
        let input = include_str!("example.txt"); 
        let (mut stacks, operations) = parse(input);
        operations.iter().for_each(|(amount, from, to)| {
            move_crates_2(&mut stacks, *amount, *from, *to);
        });
        let result = result(&stacks);
        assert_eq!(result, "MCD");
    }
    
    #[test]
    fn solve_input_2() {
        let input = include_str!("input.txt"); 
        let (mut stacks, operations) = parse(input);
        operations.iter().for_each(|(amount, from, to)| {
            move_crates_2(&mut stacks, *amount, *from, *to);
        });
        let result = result(&stacks);
        eprintln!("Result2: {}", result);
    }
}
