use regex::Regex;

pub fn day3() {
    let sides = parse();

    let part1_ans = part1(&sides);
    assert_eq!(part1_ans, 1050);

    let part2_ans = part2(&sides);
    assert_eq!(part2_ans, 1921);
}

fn part1(sides: &Vec<Vec<i64>>) -> usize {
    sides
        .into_iter()
        .filter(|side| {
            let mut side: Vec<i64> = (*side).clone();
            side.sort();
            side[0] + side[1] > side[2]
        })
        .count()
}

fn part2(sides: &Vec<Vec<i64>>) -> usize {
    let mut count = 0;
    for i in 0..3 {
        for j in (0..sides.len() - 2).step_by(3) {
            let mut side = vec![sides[j][i], sides[j + 1][i], sides[j + 2][i]];
            side.sort();
            if side[0] + side[1] > side[2] {
                count += 1;
            }
        }
    }
    count
}

fn parse() -> Vec<Vec<i64>> {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    let re = Regex::new(r"\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    re.captures_iter(&input)
        .map(|capture| {
            let s1 = capture.get(1).unwrap().as_str().parse().unwrap();
            let s2 = capture.get(2).unwrap().as_str().parse().unwrap();
            let s3 = capture.get(3).unwrap().as_str().parse().unwrap();
            vec![s1, s2, s3]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_test() {
        day3();
    }
}
