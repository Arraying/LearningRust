use std::fs::read_to_string;


fn main() {
    let p1: i32 = read_lines("input/input.txt")
        .iter()
        .map(get_digits_part1)
        .map(get_number)
        .sum();
    let p2: i32 = read_lines("input/input.txt")
        .iter()
        .map(get_digits_part2)
        .map(get_number)
        .sum();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn get_number(v: Vec<char>) -> i32 {
    let first = v[0];
    let last = v[v.len() - 1];
    let mut res = String::new();
    res.push(first);
    res.push(last);
    res.parse::<i32>().unwrap()
}

fn get_digits_part2(v: &String) -> Vec<char> {
    let mut work = String::from(v);
    let mut results: Vec<char> = Vec::new();
    get_digits_part2_remover(&mut work, &mut results);
    results
}

fn get_digits_part2_remover(v: &mut String, result: &mut Vec<char>) {
    if v.is_empty() {
        return;
    }
    let x = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i_minus_1, prefix) in x.iter().enumerate() {
        if v.starts_with(prefix) {
            let the_char = char::from_digit((i_minus_1 + 1).try_into().unwrap(), 10).unwrap();
            result.push(the_char);
            v.remove(0);
            get_digits_part2_remover(v, result);
            return;
        }
    }
    let popped = v.remove(0);
    // Can't use if let because it's "unstable".
    match popped {
        i if i.is_digit(10) => result.push(i),
        _ => {}
    }
    get_digits_part2_remover(v, result);
}


fn get_digits_part1(str: &String) -> Vec<char> {
    str.chars()
        .filter(|c| c.is_digit(10))
        .collect()
}


fn read_lines(file_name: &str) -> Vec<String> {
    read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}