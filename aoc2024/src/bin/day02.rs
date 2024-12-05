use std::cmp::Ordering;

fn part1(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .filter(|line| {
            let samples = parse_list!(line, isize);
            if samples.is_empty() || !monotonic(samples.iter()) {
                return false;
            }
            samples
                .windows(2)
                .map(|window| window[0].abs_diff(window[1]))
                .all(|delta| delta >= 1 && delta <= 3 && delta != 0)
        })
        .count()
}

fn part2(input: &str) -> impl std::fmt::Display {
    input
        .lines()
        .filter(|line| {
            let master_samples = parse_list!(line, isize);
            if master_samples.is_empty() {
                return false;
            }
            (0..master_samples.len())
                .map(|index| {
                    let mut samples = master_samples.clone();
                    if index < samples.len() {
                        samples.remove(index);
                    }
                    if samples.is_empty() || !monotonic(samples.iter()) {
                        return false;
                    }
                    samples
                        .windows(2)
                        .map(|window| window[0].abs_diff(window[1]))
                        .all(|delta| delta >= 1 && delta <= 3 && delta != 0)
                })
                .any(|v| v)
        })
        .count()
}

/// Determines whether an iterator produces monotonically increasing,
/// or monotonically decreasing values.
fn monotonic<I, T>(mut iterator: I) -> bool
where
    I: Iterator<Item = T>,
    T: Copy + Ord,
{
    let mut first = None;
    let mut has_increased = false;
    let mut has_decreased = false;

    while let Some(next) = iterator.next() {
        let Some(prev) = first.replace(next) else {
            continue;
        };

        match next.cmp(&prev) {
            Ordering::Equal => continue,
            Ordering::Less => {
                has_decreased = true;
                if has_increased {
                    return false;
                }
            }
            Ordering::Greater => {
                has_increased = true;
                if has_decreased {
                    return false;
                }
            }
        }
    }

    true
}

aoc::aoc!(day02, "2", "4");
