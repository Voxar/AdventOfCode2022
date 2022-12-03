use std::{fs::read_to_string, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = read_to_string("input.txt")?;
    let input = parse_input(raw_input.as_str());
        
    println!("Part 1 result: {}", part_1(&input));
    println!("Part 2 result: {}", part_2(&input));
    Ok(())
}

#[derive(PartialEq, Copy, Clone)]
enum Weapon {
    Rock, Paper, Scissor
}


impl Weapon {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3
        }
    }
    fn beats(&self) -> Weapon {
        match self {
            Self::Rock => Weapon::Scissor,
            Self::Paper => Weapon::Rock,
            Self::Scissor => Weapon::Paper
        }
    }
    fn beaten_by(&self) -> Weapon {
        match self {
            Self::Rock => Weapon::Paper,
            Self::Paper => Weapon::Scissor,
            Self::Scissor => Weapon::Rock
        }
    }
}

impl TryFrom<char> for Weapon {
    type Error = String;

    fn try_from(value: char) -> Result<Weapon, Self::Error> {
        match value {
            'A' | 'X' => Ok(Weapon::Rock),
            'B' | 'Y' => Ok(Weapon::Paper),
            'C' | 'Z' => Ok(Weapon::Scissor),
            e => Err(format!("char '{e}' is not a weapon choice"))
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Outcome {
    Win, Draw, Lose
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            e => Err(format!("char '{e}' is not an outcome"))
        }
    }
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input
    .trim()
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> (char, char) {
    let split = line.split_ascii_whitespace().collect::<Vec<_>>();
    return (
        split[0].chars().next().unwrap(), 
        split[1].chars().next().unwrap()
    )
}

fn part_1(input: &Vec<(char, char)>) -> u32 {
    input
        .iter().map(|(a, b)| (
            Weapon::try_from(*a).unwrap(), 
            Weapon::try_from(*b).unwrap()
        ))
        .fold(0, |score, (opponent, me)| {
            let outcome = calc_outcome(me, opponent);
            score + outcome.score() + me.score()
        })
}

fn calc_outcome(me: Weapon, opponent: Weapon) -> Outcome {
    if me.beats() == opponent {
        Outcome::Win
    } else if opponent.beats() == me {
        Outcome::Lose
    } else {
        Outcome::Draw
    }
}

fn part_2(input: &Vec<(char, char)>) -> u32 {
    input
        .iter().map(|(a, b)| { (
            Weapon::try_from(*a).unwrap(),
            Outcome::try_from(*b).unwrap()
        ) })
        .fold(0, |score, (opponent, outcome)| {
            let me = calc_move(opponent, outcome);
            score + outcome.score() + me.score()
        })
}

fn calc_move(opponent: Weapon, outcome: Outcome) -> Weapon {
    if outcome == Outcome::Win {
        opponent.beaten_by()
    } else if outcome == Outcome::Lose {
        opponent.beats()
    } else {
        opponent
    }
}

#[test]
fn test() {
    let input = r#"
    A Y
    B X
    C Z
    "#.trim();

    assert_eq!(part_1(&parse_input(input)), 15);
    assert_eq!(part_2(&parse_input(input)), 12);
}