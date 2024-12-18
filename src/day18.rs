use aoc_runner_derive::aoc;
use smallvec::SmallVec;

use std::{cmp::Ordering, collections::BinaryHeap};

//const N: u8 = 7;
const N: u8 = 71;
const LENGTH: usize = N as usize * N as usize;
//const BYTES_TO_CHECK: usize = 12;
const BYTES_TO_CHECK: usize = 1024;

#[derive(Debug)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[inline(always)]
fn set_corrupted(v: u16) -> u16 {
    (1 << 15) | v
}

#[inline(always)]
fn is_corrupted(v: u16) -> bool {
    (v >> 15) & 1 == 1
}

#[inline(always)]
fn set_coordinates(x: u8, y: u8) -> u16 {
    ((x as u16) << 8) | y as u16
}

#[inline(always)]
fn get_coordinates(v: u16) -> (u8, u8) {
    (((v >> 8) & ((1 << 7) - 1)) as u8, (v & 0xff) as u8)
}

#[inline(always)]
fn coordinates_idx(x: u8, y: u8) -> usize {
    ((y as usize) * (N as usize)) + (x as usize)
}

fn left(v: u16) -> Option<(u16, Direction)> {
    let (x, y) = get_coordinates(v);
    x.checked_sub(1)
        .map(|x| (set_coordinates(x, y), Direction::Left))
}

fn right(v: u16) -> Option<(u16, Direction)> {
    let (x, y) = get_coordinates(v);
    let x = x + 1;
    if x >= N {
        None
    } else {
        Some((set_coordinates(x, y), Direction::Right))
    }
}

fn bottom(v: u16) -> Option<(u16, Direction)> {
    let (x, y) = get_coordinates(v);
    let y = y + 1;
    if y >= N {
        None
    } else {
        Some((set_coordinates(x, y), Direction::Bottom))
    }
}

fn top(v: u16) -> Option<(u16, Direction)> {
    let (x, y) = get_coordinates(v);
    y.checked_sub(1)
        .map(|y| (set_coordinates(x, y), Direction::Top))
}

fn adjacent(v: u16) -> [Option<(u16, Direction)>; 4] {
    [top(v), right(v), bottom(v), left(v)]
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vertex(usize, u16);

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[inline(always)]
fn manhattan(v1: u16, v2: u16) -> u16 {
    let (x1, y1) = get_coordinates(v1);
    let (x2, y2) = get_coordinates(v2);

    (u8::abs_diff(x1, x2) + u8::abs_diff(y1, y2)) as u16
}

fn a_star(graph: &[u16]) -> Option<u16> {
    let start = 0;
    let goal = LENGTH - 1;

    let mut parent = [None; LENGTH];
    let mut costs = [None; LENGTH];
    costs[start] = Some(0);

    let mut frontier = BinaryHeap::new();
    frontier.push(Vertex(start, 0));

    while let Some(Vertex(this, _)) = frontier.pop() {
        //        println!("Vertex: {this} ({})", graph[this]);

        if this == goal {
            //            println!("Found {} !", goal);
            break;
        }

        for idx in adjacent(graph[this])
            .into_iter()
            .flatten()
            .filter_map(|(n, _)| {
                let (x, y) = get_coordinates(n);
                let idx = coordinates_idx(x, y);
                if is_corrupted(graph[idx]) {
                    None
                } else {
                    Some(idx)
                }
            })
        {
            let cost = unsafe { costs[this].unwrap_unchecked() } + 1;

            let to_explore = match costs[idx] {
                None => true,
                Some(neighbor_cost) => cost < neighbor_cost,
            };

            if to_explore {
                costs[idx] = Some(cost);
                frontier.push(Vertex(idx, cost + manhattan(graph[goal], graph[idx])));
                parent[idx] = Some(this);
            }
        }
    }

    //    let mut vertex = goal;
    //    while let Some(parent) = parent[vertex] {
    //        println!("{parent}");
    //        vertex = parent;
    //    }
    costs[goal]
}

fn get_bad_blocks(blocks: &str) -> impl Iterator<Item = (u8, u8)> + use<'_> {
    blocks
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(x, y)| unsafe {
            (
                x.parse::<u8>().unwrap_unchecked(),
                y.parse::<u8>().unwrap_unchecked(),
            )
        })
}

fn print_mem(mem: &[u16]) {
    println!();
    for loc in mem {
        print!("{:?} {} -> ", get_coordinates(*loc), is_corrupted(*loc));
        for (adj, dir) in adjacent(*loc).into_iter().flatten() {
            print!("{:?}: {:?} ", dir, get_coordinates(adj));
        }
        println!();
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u16 {
    let mut mem = SmallVec::<[u16; LENGTH]>::new();

    for y in 0..N {
        for x in 0..N {
            mem.push(set_coordinates(x, y));
        }
    }

    for (x, y) in get_bad_blocks(input).take(BYTES_TO_CHECK) {
        let loc = set_coordinates(x, y);
        mem[coordinates_idx(x, y)] = set_corrupted(loc);
    }

    //    print_mem(&mem);

    unsafe { a_star(&mem).unwrap_unchecked() }
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> &'static str {
    let mut byte_coordinates = "";

    let mut mem = SmallVec::<[u16; LENGTH]>::new();

    for y in 0..N {
        for x in 0..N {
            mem.push(set_coordinates(x, y));
        }
    }

    let mut bad_blocks = get_bad_blocks(input);

    for (x, y) in bad_blocks.by_ref().take(BYTES_TO_CHECK) {
        let loc = set_coordinates(x, y);
        mem[coordinates_idx(x, y)] = set_corrupted(loc);
    }

    for (x, y) in bad_blocks.by_ref() {
        let loc = set_coordinates(x, y);
        mem[coordinates_idx(x, y)] = set_corrupted(loc);
        if a_star(&mem).is_none() {
            byte_coordinates = format!("{x},{y}").leak();
            break;
        }
    }

    byte_coordinates
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0
        "};
        assert_eq!(part1(input), 22);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0
        "};
        assert_eq!(part2(input), "6,1");
    }
}
