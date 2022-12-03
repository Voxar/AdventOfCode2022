use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt")
        .expect("should read input file");
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> u32 {
            let line = line.trim();
            let (left, right) = line.split_at(line.len()/2);
            let left: HashSet<char> = left.chars().collect();
            let right: HashSet<char> = right.chars().collect();
            let common = left.intersection(&right);
            common.fold(0, |score, char| {
                score + prio(char)
            })
        })
        .sum()
}

fn prio(equipment: &char) -> u32 {
    if equipment.is_uppercase() {
        27 + *equipment as u32 - 'A' as u32
    } else {
        1 + *equipment as u32 - 'a' as u32
    }
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunks| -> u32 {
            let sets = chunks
                .into_iter()
                .map(|chunk| chunk.trim().chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>();
            
            let common = sets
                .iter()
                .skip(1)
                .fold(sets[0].clone(), |acc, set| 
                    acc.intersection(set).cloned().collect::<HashSet<char>>()
                );
            common
                .iter()
                .map(prio)
                .sum()
        }).sum()
}


#[test]
fn test() {
    let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"#;

    assert_eq!(157, part_1(input));
    assert_eq!(70, part_2(input));
}
