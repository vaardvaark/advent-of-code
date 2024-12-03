fn main() {
    let input = std::io::read_to_string(std::io::stdin())
        .expect("puzzle input should be provided on standard input");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> impl std::fmt::Display {
    0
}

fn part2(input: &str) -> impl std::fmt::Display {
    0
}

#[cfg(test)]
mod day04 {

    const EXAMPLE: &str = r#""#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(EXAMPLE).to_string(), "0");
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(EXAMPLE).to_string(), "0");
    }
}
