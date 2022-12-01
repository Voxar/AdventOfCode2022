// https://adventofcode.com/2022/day/1


fn main() {
    let mut max = 0;
    std::fs::read_to_string("input.txt")
        .expect("Shoud read the input file")
        .lines()
        .map(&str::parse::<u32>)
        .reduce(|acc, r|
            match r {
                Ok(val) => Ok(acc.unwrap_or(0) + val),
                Err(e) => {
                    let v = acc.unwrap_or(0);
                    max = if max > v { max } else { v };
                    Err(e)
                }
            }
        )
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .expect("a result");
    print!("The fattest elf has {} calories.", max);
}
