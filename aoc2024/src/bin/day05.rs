use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    rc::Rc,
};

aoc::main!();

struct Updates {
    updates: Vec<Vec<usize>>,
    rules: Rc<HashMap<usize, HashSet<usize>>>,
}

impl Updates {
    fn extract(input: &str) -> Self {
        let mut lines = input.lines().map(|line| line.trim());
        let mut rules: HashMap<_, HashSet<_>> = HashMap::new();
        for (left, right) in take_pairs!(&mut lines, '|') {
            rules.entry(left).or_default();
            rules
                .entry(right)
                .and_modify(|before_set| {
                    before_set.insert(left);
                })
                .or_insert([left].into());
        }
        Self {
            rules: Rc::new(rules),
            updates: take_lists!(&mut lines, ',').collect(),
        }
    }

    fn is_valid(&self, update: &[usize]) -> bool {
        update
            .iter()
            .scan(HashSet::with_capacity(update.len()), |seen, page| {
                if !seen.iter().all(|n| self.rules[page].contains(n)) {
                    return Some(false);
                }
                seen.insert(*page);
                Some(true)
            })
            .all(|v| v)
    }

    fn valid(&self) -> Self {
        Self {
            updates: self
                .updates
                .iter()
                .filter(|update| self.is_valid(update))
                .cloned()
                .collect(),
            rules: Rc::clone(&self.rules),
        }
    }

    fn invalid(&self) -> Self {
        Self {
            updates: self
                .updates
                .iter()
                .filter(|update| !self.is_valid(update))
                .cloned()
                .collect(),
            rules: Rc::clone(&self.rules),
        }
    }

    fn sum_middles(&self) -> usize {
        self.updates
            .iter()
            .map(|update| {
                assert_eq!(update.len() % 2, 1);
                update[update.len() / 2]
            })
            .sum()
    }

    fn make_valid(mut self) -> Self {
        for update in &mut self.updates {
            update.sort_by(|a, b| {
                if self.rules.get(a).is_some_and(|set| set.contains(b)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
        }
        self
    }
}

fn part1(input: &str) -> impl std::fmt::Display {
    let updates = Updates::extract(input);
    updates.valid().sum_middles()
}

fn part2(input: &str) -> impl std::fmt::Display {
    let updates = Updates::extract(input);
    updates.invalid().make_valid().sum_middles()
}

#[cfg(test)]
mod day05 {
    const EXAMPLE: &str = r#"47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"#;

    #[test]
    fn part1() {
        assert_eq!(super::part1(EXAMPLE).to_string(), "143");
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(EXAMPLE).to_string(), "123");
    }
}
