use aoc::*;

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (result, params) = line.split_once(':').unwrap();
            let result = parse(result);
            let params = params.split_whitespace().map(parse).collect();
            (result, params)
        })
        .collect()
}

fn check(target: u64, params: &[u64], include_concat: bool) -> bool {
    let p = *match params.last() {
        None => return false,
        Some(value) if value == &target => return true,
        Some(parameter) => parameter,
    };

    if target.rem_euclid(p) == 0 && check(target / p, &params[..params.len() - 1], include_concat) {
        return true;
    }

    if include_concat {
        let base = 10u64.pow(p.ilog10() + 1);
        if target
            .checked_sub(p)
            .is_some_and(|v| v.rem_euclid(base) == 0)
            && check(
                (target - p) / base,
                &params[..params.len() - 1],
                include_concat,
            )
        {
            return true;
        }
    }

    target
        .checked_sub(p)
        .is_some_and(|v| check(v, &params[..params.len() - 1], include_concat))
}

fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter_map(|(target, params)| check(*target, params, false).then_some(target))
        .sum()
}

fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter_map(|(target, params)| check(*target, params, true).then_some(target))
        .sum()
}

aoc::setup! {
    day07, parse_input;
    part1 == 3749,
    part2 == 11387
}
