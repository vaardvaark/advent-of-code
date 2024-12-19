#![allow(clippy::mut_range_bound)]

use std::{cmp::Reverse, collections::BinaryHeap};

type Parsed = (
    Vec<usize>,
    Vec<BinaryHeap<Reverse<usize>>>,
    Vec<(usize, usize)>,
);

const FREE: usize = usize::MAX;

fn parse_input(input: &str) -> Parsed {
    let mut output = Vec::with_capacity(input.len());
    let mut free = vec![BinaryHeap::new(); 10];
    let mut file_map = vec![];
    let mut location: usize = 0;
    for (id, length) in input
        .as_bytes()
        .iter()
        .filter(|b| b.is_ascii_digit())
        .enumerate()
    {
        let length = (length - b'0') as usize;
        if length == 0 {
            continue;
        }

        if id % 2 == 0 {
            file_map.push((location, length));
            output.extend_from_slice(&vec![id / 2; length]);
        } else {
            free[length].push(Reverse(location));
            output.extend_from_slice(&vec![FREE; length]);
        }
        location += length;
    }
    free[0] = BinaryHeap::new();
    (output, free, file_map)
}

fn part1((file_map, _, _): &Parsed) -> impl std::fmt::Display {
    let mut file_map = file_map.clone();
    let mut base = file_map
        .iter()
        .position(|&v| v == FREE)
        .unwrap_or(file_map.len() - 1);

    'next: for idx in (0..file_map.len()).rev() {
        if file_map[idx] == FREE {
            continue;
        }

        for free_idx in base..idx {
            if file_map[free_idx] == FREE {
                file_map.swap(idx, free_idx);
                base = free_idx;
                continue 'next;
            }
        }

        // We didn't find any free blocks.
        break;
    }

    file_map
        .iter()
        .enumerate()
        .map(
            |(position, id)| {
                if *id == FREE {
                    0
                } else {
                    *id * position
                }
            },
        )
        .sum::<usize>()
}

fn part2((disk_map, free_map, file_map): &Parsed) -> impl std::fmt::Display {
    let mut disk_map = disk_map.clone();
    let mut free_map = free_map.clone();

    for &(file_location, file_length) in file_map.iter().rev() {
        if file_length == 0 {
            continue;
        }

        let mut best_loc = FREE;
        let mut best_len = 0;
        for (free_length, free_heap) in free_map.iter().enumerate().take(10).skip(file_length) {
            if let Some(&Reverse(free_location)) = free_heap.peek() {
                if free_location < file_location && free_location < best_loc {
                    best_loc = free_location;
                    best_len = free_length;
                }
            }
        }

        if best_loc != FREE {
            free_map[best_len].pop();
            for offset in 0..file_length {
                disk_map.swap(file_location + offset, best_loc + offset);
            }

            let remaining = best_len - file_length;
            if remaining > 0 {
                free_map[remaining].push(Reverse(best_loc + file_length));
            }
        }
    }

    disk_map
        .iter()
        .enumerate()
        .map(
            |(position, id)| {
                if *id == FREE {
                    0
                } else {
                    *id * position
                }
            },
        )
        .sum::<usize>()
}

aoc::setup! {
    day09, parse_input;
    part1 == 1928,
    part2 == 2858
}
