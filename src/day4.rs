use itertools::Itertools;
use regex::Regex;
use std::collections::*;

#[derive(Debug)]
struct Room {
    name: String,
    id: i64,
    checksum: String,
}

pub fn day4() {
    let input = parse();

    let part1_ans = part1(&input);
    assert_eq!(part1_ans, 173787);
}

fn part1(input: &Vec<Room>) -> i64 {
    input
        .into_iter()
        .filter(|room| most_common(&room.name) == room.checksum)
        .map(|room| room.id)
        .sum()
}

fn most_common(name: &str) -> String {
    let mut counter = HashMap::new();
    name.chars()
        .filter(|c| *c != '-')
        .for_each(|c| *counter.entry(c).or_insert(0) += 1);
    dbg!(&counter);

    let s = counter
        .into_iter()
        .sorted_by(|e1, e2| e1.1.cmp(&e2.1).reverse().then_with(|| e1.0.cmp(&e2.0)))
        .map(|(c, _)| c)
        .take(5)
        .collect();
    dbg!(&s);
    s
}

fn parse() -> Vec<Room> {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let re = Regex::new(r"((?:\w+-?)+)-(\d+)\[(\w+)\]").unwrap();

    re.captures_iter(&input)
        .map(|capture| {
            let name = capture.get(1).unwrap().as_str().to_owned();
            let id = capture.get(2).unwrap().as_str().parse().unwrap();
            let checksum = capture.get(3).unwrap().as_str().to_owned();

            Room { name, id, checksum }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test() {
        day4();
    }
}
