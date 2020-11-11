use super::common::*;
use std::collections::*;
use std::fs;

#[derive(Debug)]
struct Instr(LR, i64);

pub fn day1() {
    let input = parse();
    let part1_dist = part1(&input);
    assert_eq!(part1_dist, 271);

    let part2_dist = part2(&input);
    assert_eq!(part2_dist, 153);
}

fn part1(input: &Vec<Instr>) -> i64 {
    let mut pos = Coords2D::new(0, 0);
    let mut dir = Coords2D::new(0, 1);
    for instr in input {
        dir = dir.rotate(instr.0);
        pos = pos + dir * instr.1;
    }
    pos.x + pos.y
}

fn part2(input: &Vec<Instr>) -> i64 {
    let mut pos = Coords2D::new(0, 0);
    let mut dir = Coords2D::new(0, 1);
    let mut visited = HashSet::new();
    visited.insert(pos);

    for instr in input {
        dir = dir.rotate(instr.0);

        for _ in 1..=instr.1 {
            pos += dir;

            if visited.contains(&pos) {
                return pos.x.abs() + pos.y.abs();
            } else {
                visited.insert(pos);
            }
        }
    }
    pos.x.abs() + pos.y.abs()
}

fn parse() -> Vec<Instr> {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    input
        .trim()
        .split(", ")
        .map(|line| {
            let val = line.get(1..).unwrap().parse().unwrap();
            if line.get(0..1).unwrap() == "L" {
                Instr(LR::Left, val)
            } else {
                Instr(LR::Right, val)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        day1();
    }
}
