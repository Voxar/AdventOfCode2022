// https://adventofcode.com/2022/day/1


fn main() {
    let input = std::fs::read_to_string("input.txt")
        .expect("Shoud read the input file")
        .split("\n\n")
        .map( |chunk| chunk
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum()
        ).collect();
    assert_eq!(part_one(&input), 68787);
    assert_eq!(part_two(&input), 198041);
}

fn part_one(input: &Vec<u32>) -> u32 {
    let top_1 = input
        .iter()
        .max()
        .unwrap();
    println!("The fattest elf has {} calories.", top_1);
    *top_1
}

fn part_two(input: &Vec<u32>) -> u32 {
    let mut elfs = input.clone();
    elfs.sort_by(|a, b| b.cmp(a) );
    let top_3 = elfs
        .into_iter()
        .take(3)
        .sum();
    println!("The three fattest elfs together has {:?} calories.", top_3);
    top_3
}

