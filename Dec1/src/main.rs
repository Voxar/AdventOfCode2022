// https://adventofcode.com/2022/day/1

fn main() {
    println!(
        "The fattest elf has {} calories.",
        std::fs::read_to_string("input.txt")
            .expect("Shoud read the input file")
            .lines()
            .fold((0, 0), |(max, cur), line| match line.parse::<u32>() {
                Ok(val) => (max, cur + val),
                Err(_) => (if max > cur { max } else { cur }, 0),
            }).0
    );
}
