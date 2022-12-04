use std::{ops::{RangeInclusive}, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt")
        .expect("should read input file");
    let input = parse_input(&input);
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

struct Pair {
    a: RangeInclusive<u32>,
    b: RangeInclusive<u32>
}

impl Pair {
    fn either_fully_contains_other(&self) -> bool {
        (self.a.contains(&self.b.start()) &&
        self.a.contains(&self.b.end())) ||
        (self.b.contains(&self.a.start()) &&
        self.b.contains(&self.a.end()))
    }
    fn overlaps(&self) -> bool {
        self.a.contains(&self.b.start()) ||
        self.a.contains(&self.b.end()) ||
        self.b.contains(&self.a.start()) ||
        self.b.contains(&self.a.end())
    }
}

fn parse_range(input: &str) -> RangeInclusive<u32> {
    let mut it = input.trim()
        .split("-")
        .map(|s| s.parse::<u32>().unwrap());
    it.next().unwrap()..=it.next().unwrap()
}

fn parse_pair(input: &str) -> Pair {
    let mut ranges = input.trim()
        .split(",")
        .map(parse_range);
    Pair {
        a: ranges.next().unwrap(),
        b: ranges.next().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    input.trim()
        .lines()
        .map(parse_pair)
        .collect::<Vec<_>>()
}

fn part_1(input: &Vec<Pair>) -> u32 {
    input
        .into_iter()
        .fold(0, |count, range| {
            count + if range.either_fully_contains_other() { 1 } else { 0 }
        })
}

fn part_2(input: &Vec<Pair>) -> u32 {
    input
        .into_iter()
        .fold(0, |count, range| {
            count + if range.overlaps() { 1 } else { 0 }
        })
}

#[test]
fn test() {
    let input = r#"2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8"#;
    let input = parse_input(input);
    assert_eq!(2, part_1(&input));
    assert_eq!(4, part_2(&input));
}