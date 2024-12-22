use aoc::*;
use std::collections::HashMap;

type KeyMap = HashMap<(u8, u8), (String, String)>;
type Parsed = (Vec<String>, KeyMap, Grid<u8>, KeyMap, Grid<u8>);

fn parse_input(input: &str) -> Parsed {
    let codes = input.lines().map(|line| line.to_owned()).collect();

    let num_keypad = gridify_ascii("789\n456\n123\n\x000A".lines());
    let dir_keypad = gridify_ascii("\x00^A\n<v>".lines());

    let num_map = build_keymap(&num_keypad);
    let dir_map = build_keymap(&dir_keypad);

    (codes, num_map, num_keypad, dir_map, dir_keypad)
}

fn build_keymap(keys: &Grid<u8>) -> KeyMap {
    fn build_map(v: Vec2, hneg: u8, hpos: u8, vneg: u8, vpos: u8) -> (String, String) {
        let mut horz = String::new();
        let mut vert = String::new();
        let hsym = if v.x.is_negative() { hneg } else { hpos };
        let vsym = if v.y.is_negative() { vneg } else { vpos };
        for _ in 0..v.x.abs() {
            horz.push(hsym as char);
        }
        for _ in 0..v.y.abs() {
            vert.push(vsym as char);
        }
        (vert, horz)
    }

    let mut keymap = HashMap::new();
    let keyset: Vec<_> = keys
        .iter_pos()
        .filter_map(|pos| keys[pos].is_ascii().then_some(keys[pos]))
        .collect();

    for &start in &keyset {
        for &end in &keyset {
            let start_pos = keys.position(|&v| v == start).unwrap();
            let end_pos = keys.position(|&v| v == end).unwrap();
            keymap.insert(
                (start, end),
                build_map(end_pos - start_pos, b'<', b'>', b'^', b'v'),
            );
        }
    }
    keymap
}

fn part1(input: &Parsed) -> impl std::fmt::Display {
    solve(3, input)
}

fn part2(input: &Parsed) -> impl std::fmt::Display {
    solve(26, input)
}

fn solve(depth: usize, (codes, nmap, nkey, dmap, dkey): &Parsed) -> usize {
    let mut result = 0;
    let mut cache = HashMap::new();
    for code in codes {
        let value: usize = parse(code.trim_end_matches('A'));
        result += value * map(&mut cache, nmap, nkey, dmap, dkey, code, depth, 0);
    }
    result
}

/// Verifies a mapping does not pass through an invalid position.
fn validate_mapping(from: u8, sequence: &str, keys: &Grid<u8>) -> bool {
    let mut cursor = keys.cursor(keys.position(|&v| v == from).unwrap());
    for m in sequence.trim_end_matches('A').bytes() {
        if !cursor.step(Cardinal::from_ascii(m).unwrap()) {
            return false;
        }
        if cursor.value() == &0 {
            return false;
        }
    }
    true
}

fn map(
    cache: &mut HashMap<(String, usize, usize), usize>,
    num_mappings: &KeyMap,
    num_keypad: &Grid<u8>,
    dir_mappings: &KeyMap,
    dir_keypad: &Grid<u8>,
    sequence: &str,
    limit: usize,
    depth: usize,
) -> usize {
    if let Some(len) = cache.get(&(sequence.to_string(), limit, depth)) {
        return *len;
    }

    if depth == limit {
        return sequence.len();
    }

    let mut current = b'A';
    let mut total = 0;
    for key in sequence.bytes() {
        let (vmap, hmap) = if depth == 0 {
            num_mappings.get(&(current, key)).unwrap()
        } else {
            dir_mappings.get(&(current, key)).unwrap()
        };

        let shortest = [format!("{vmap}{hmap}A"), format!("{hmap}{vmap}A")]
            .iter()
            .filter(|seq| {
                if depth == 0 {
                    validate_mapping(current, seq, num_keypad)
                } else {
                    validate_mapping(current, seq, dir_keypad)
                }
            })
            .map(|path| {
                map(
                    cache,
                    num_mappings,
                    num_keypad,
                    dir_mappings,
                    dir_keypad,
                    path,
                    limit,
                    depth + 1,
                )
            })
            .min()
            .unwrap();

        total += shortest;
        current = key;
    }

    cache.insert((sequence.to_string(), limit, depth), total);
    total
}

aoc::setup! {
    day21, parse_input;
    part1 == 126384,
    part2 == 154115708116294_u64
}
