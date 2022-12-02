
fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    let mut current = None;
    for line in input.lines() {
        if line.len() == 0 {
            if let Some(c) = current.take() {
                output.push(c);
            }            
        } else {
            if current.is_none() {
                current = Some(vec![]);
            }

            match line.parse::<i32>() {
                Ok(v) => current.as_mut().unwrap().push(v),
                Err(e) => panic!("{} -> '{}'", e, line),
            }
        }
    }

    if let Some(c) = current.take() {
        output.push(c);
    }    

    output
}

fn find_elf_carrying_most(elves: Vec<Vec<i32>>) -> i32 {
    elves.iter().map(|elf| elf.iter().sum::<i32>()).max().unwrap()
}

fn find_top_carrying_elves(elves: Vec<Vec<i32>>, places: usize) -> Vec<i32> {
    let mut max_counts = elves.iter().map(|elf| elf.iter().sum::<i32>()).collect::<Vec<_>>();
    max_counts.sort();
    let len = max_counts.len();
    max_counts.as_slice()[len-places..len].into()
}

#[cfg(test)]
mod day1 {
    use super::*;
    #[test]
    fn parse_example() {
        let input = include_str!("example.txt"); 
        let elves = parse(input);
        assert_eq!(find_elf_carrying_most(elves), 24000);
    }

    #[test]
    fn solve_input_1() {
        let input = include_str!("input.txt"); 
        let elves = parse(input);
        let max = find_elf_carrying_most(elves);
        println!("Elf carrying most, carries {} calories", max);
    }

    #[test]
    fn solve_example_2() {
        let input = include_str!("example.txt"); 
        let elves = parse(input);
        println!("{:?}", elves);
        let top_elves = find_top_carrying_elves(elves, 3);
        println!("{:?}", top_elves.as_slice());
        assert!(top_elves.as_slice() == &[10000, 11000, 24000]);
    }
    
    #[test]
    fn solve_input_2() {
        let input = include_str!("input.txt"); 
        let elves = parse(input);
        let top_elves = find_top_carrying_elves(elves, 3);
        println!("{:?}", top_elves.iter().sum::<i32>());
    }
}
