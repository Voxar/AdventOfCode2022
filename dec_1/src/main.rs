// https://adventofcode.com/2022/day/1


fn main() {
    assert_eq!(part_one(), 68787);
    assert_eq!(part_two(), 198041);
}

fn part_one() -> u32 {
    let top_1 = std::fs::read_to_string("input.txt")
        .expect("Shoud read the input file")
        .lines()
        .fold((0, 0), |(max, cur), line| match line.parse::<u32>() {
            Ok(val) => (max, cur + val),
            Err(_) => (if max > cur { max } else { cur }, 0),
        }).0;
    println!("The fattest elf has {} calories.", top_1);
    top_1
}

fn part_two() -> u32 {
    let mut elfs = std::fs::read_to_string("input.txt")
        .expect("Shoud read the input file")
        .lines()
        .fold((Vec::new(), 0), |(mut elfs, cur), line| match line.parse::<u32>() {
            Ok(val) => (elfs, cur + val),
            Err(_) => {
                elfs.push(cur);
                (elfs, 0)
            }
        }).0;
    elfs.sort_by(|a, b| b.cmp(a) );
    let top_3 = elfs.split_at(3).0.iter().fold(0, |a, b| a + b);
    println!("The three fattest elfs together has {:?} calories.", top_3);
    top_3
}

