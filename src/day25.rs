use core::str;

use aoc_runner_derive::aoc;
use smallvec::SmallVec;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> u32 {
    let mut locks = SmallVec::<[&[u8]; 64]>::new();
    let mut keys = SmallVec::<[&[u8]; 64]>::new();

    for (i, kind) in input.bytes().enumerate().step_by((6 * 7) + 1) {
        match kind {
            b'#' => {
                //println!("lock at {i}");
                let lock = &input.as_bytes()[i..i + (6 * 7)];
                // println!("{lock:?}");
                locks.push(lock);
            }
            b'.' => {
                //println!("key at {i}");
                let key = &input.as_bytes()[i..usize::min(i + (6 * 7), input.len())];
                // println!("{key:?}");
                keys.push(key);
            }
            _ => {}
        }
    }

    let mut no_overlap = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            //println!("Trying key {} with lock {}", k, l);
            let pins = [
                6, 12, 18, 24, 30, 7, 13, 19, 25, 31, 8, 14, 20, 26, 32, 9, 15, 21, 27, 33, 10, 16,
                22, 28, 34,
            ];

            let overlaps = pins
                .into_iter()
                .filter(|&pin| key[pin] == b'#' && key[pin] == lock[pin])
                .count();

            if overlaps == 0 {
                no_overlap += 1;
            }
        }
    }

    no_overlap
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            #####
            .####
            .####
            .####
            .#.#.
            .#...
            .....
            
            #####
            ##.##
            .#.##
            ...##
            ...#.
            ...#.
            .....
            
            .....
            #....
            #....
            #...#
            #.#.#
            #.###
            #####
            
            .....
            .....
            #.#..
            ###..
            ###.#
            ###.#
            #####
            
            .....
            .....
            .....
            #....
            #.#..
            #.#.#
            #####
        "};
        assert_eq!(part1(input), 3);
    }
}
