use std::collections::{HashMap, HashSet};

type Parsed<'a> = Vec<(&'a str, &'a str)>;

fn parse_input(input: &str) -> Parsed {
    input.lines().filter_map(|s| s.split_once('-')).collect()
}

fn part1(pairs: &Parsed) -> impl std::fmt::Display {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in pairs {
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }

    let mut systems: HashSet<[&str; 3]> = HashSet::new();
    for &key in map.keys().filter(|key| key.starts_with('t')) {
        let connected = &map[key];
        if connected.len() < 2 {
            continue;
        }

        for i in 0..connected.len() {
            for j in i + 1..connected.len() {
                if i < j {
                    let a = connected[i];
                    let b = connected[j];
                    if map[a].contains(&b) {
                        let mut set = [key, a, b];
                        set.sort();
                        systems.insert(set);
                    }
                }
            }
        }
    }

    systems.len()
}

fn part2(pairs: &Parsed) -> impl std::fmt::Display {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in pairs {
        map.entry(a).or_default().push(b);
        map.entry(b).or_default().push(a);
    }

    let mut networks: Vec<HashSet<&str>> =
        map.keys().map(|&key| HashSet::from_iter([key])).collect();

    for network in networks.iter_mut() {
        for comp in map.keys() {
            if network.iter().all(|node| map[comp].contains(node)) {
                network.insert(comp);
            }
        }
    }

    let mut clique: Vec<_> = networks
        .into_iter()
        .max_by_key(|network| network.len())
        .unwrap()
        .into_iter()
        .collect();

    clique.sort();
    clique.join(",")
}

aoc::setup! {
    day23, parse_input;
    part1 == 7,
    part2 == "co,de,ka,ta"
}
