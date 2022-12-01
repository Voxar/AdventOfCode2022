// https://adventofcode.com/2022/day/1

fn main() {
    let input = read_input_inline();
    assert_eq!(part_one(&input), 68787);
    assert_eq!(part_two(&input), 198041);
}

#[allow(dead_code)]
fn read_input_chunks() -> Vec<u32> {
    std::fs::read_to_string("input.txt")
        .expect("Shoud read the input file")
        // Split by elf
        .split("\n\n")
        .map( |chunk| chunk
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum()
        ).collect()
}

#[allow(dead_code)]
fn read_input_inline() -> Vec<u32> {
    std::fs::read_to_string("input.txt")
        .expect("Shoud read the input file")
        .lines()
        .fold((Vec::new(), 0), |(mut elfs, cur), line| match line.parse::<u32>() {
            Ok(val) => (elfs, cur + val),
            Err(_) => {
                elfs.push(cur);
                (elfs, 0)
            }
        }).0
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

