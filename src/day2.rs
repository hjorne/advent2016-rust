use std::fs;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn day2() {
    let input = parse();

    let part1_ans = part1(&input);
    assert_eq!(part1_ans, "48584");

    let part2_ans = part2(&input);
    assert_eq!(part2_ans, "563B6");
}

fn part1(input: &Vec<Vec<Dir>>) -> String {
    let keypad = vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];
    let x_max = keypad.len() as i64;
    let y_max = keypad[0].len() as i64;

    let (s, (_, _)) = input
        .into_iter()
        .fold(("".to_owned(), (1, 1)), |(mut s, (x, y)), dir| {
            let (x_new, y_new) = dir
                .into_iter()
                .fold((x, y), |r, dir| dir2diff(dir, r, (x_max, y_max)));
            s.push_str(keypad[x_new as usize][y_new as usize]);
            (s, (x_new, y_new))
        });

    s
}

fn part2(input: &Vec<Vec<Dir>>) -> String {
    let keypad = vec![
        vec![" ", " ", "1", " ", " "],
        vec![" ", "2", "3", "4", " "],
        vec!["5", "6", "7", "8", "9"],
        vec![" ", "A", "B", "C", " "],
        vec![" ", " ", "D", " ", " "],
    ];

    let (s, (_, _)) = input
        .into_iter()
        .fold(("".to_owned(), (1, 1)), |(mut s, (x, y)), dir| {
            let (x_new, y_new) = dir
                .into_iter()
                .fold((x, y), |r, dir| dir2tri(dir, r, &keypad));
            s.push_str(keypad[x_new as usize][y_new as usize]);
            (s, (x_new, y_new))
        });

    s
}

fn dir2diff(dir: &Dir, (x, y): (i64, i64), (x_max, y_max): (i64, i64)) -> (i64, i64) {
    let (dx, dy) = match dir {
        Dir::Up => (-1, 0),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
        Dir::Right => (0, 1),
    };

    (trunc(x + dx, (0, x_max - 1)), trunc(y + dy, (0, y_max - 1)))
}

fn dir2tri(dir: &Dir, (x, y): (i64, i64), keypad: &Vec<Vec<&str>>) -> (i64, i64) {
    let (dx, dy) = match dir {
        Dir::Up => (-1, 0),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
        Dir::Right => (0, 1),
    };
    let x_new = trunc(x + dx, (0, keypad.len() as i64 - 1));
    let y_new = trunc(y + dy, (0, keypad[0].len() as i64 - 1));

    if keypad[x_new as usize][y_new as usize] == " " {
        (x, y)
    } else {
        (x_new, y_new)
    }
}

fn trunc(x: i64, (x_min, x_max): (i64, i64)) -> i64 {
    if x < x_min {
        x_min
    } else if x > x_max {
        x_max
    } else {
        x
    }
}

fn parse() -> Vec<Vec<Dir>> {
    fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .split_whitespace()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => panic!("Unrecognized character: {}", c),
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_test() {
        day2();
    }
}
