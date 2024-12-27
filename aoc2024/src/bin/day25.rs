use aoc::gridify_ascii;

type Set = [u8; 5];
type Parsed = (Vec<Set>, Vec<Set>);

fn parse_input(input: &str) -> Parsed {
    let mut locks = vec![];
    let mut keys = vec![];

    for block in input.split("\n\n") {
        let block = gridify_ascii(block.lines());
        let list = if block.iter_row(0).all(|&v| v == b'#') {
            &mut locks
        } else {
            &mut keys
        };

        let mut comb = [0, 0, 0, 0, 0];
        for x in 0..5 {
            comb[x] = (1..6).filter(|&y| block[(x, y)] == b'#').count() as u8;
        }
        list.push(comb);
    }

    (locks, keys)
}

fn part1((locks, keys): &Parsed) -> usize {
    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| check_key(lock, key)).count())
        .sum()
}

fn part2(_: &Parsed) -> impl std::fmt::Display {
    ""
}

fn check_key(lock: &Set, key: &Set) -> bool {
    for position in 0..5 {
        if lock[position] + key[position] > 5 {
            return false;
        }
    }
    true
}

aoc::setup! {
    day25, parse_input;
    part1 == 3,
    part2 == ""
}
