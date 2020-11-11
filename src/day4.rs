use regex::Regex;

struct Room {
    name: String,
    id: i64,
    checksum: String,
}

pub fn day4() {
    let input = parse();
}

fn parse() -> Vec<Room> {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    //let re = Regex::new(r"((?:\w+-?)+)-(\d+)\[(\w+)\]").unwrap();
    let re = Regex::new(r"\w+").unwrap();

    re.captures_iter(&input).for_each(|capture| {
        dbg!(&capture);
        ()
    });

    vec![]
}
