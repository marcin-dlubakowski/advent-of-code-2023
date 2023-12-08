use regex::*;
use std::collections::HashMap;

type Mapping = HashMap<String, (String, String)>;

fn parse_data() -> (String, Mapping) {
    let data: Vec<String> = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(String::from)
        .collect();

    let instructions = data.first().unwrap().clone();

    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    let mappings = data
        .iter()
        .skip(2)
        .map(|line| {
            let result = re.captures(line).unwrap();
            (
                result[1].to_string(),
                (result[2].to_string(), result[3].to_string()),
            )
        })
        .collect();

    (instructions, mappings)
}

fn part_one() {
    let (instructions, mappings) = parse_data();
    let mut current = String::from("AAA");
    let len = instructions.len();
    let mut i = 0;
    let mut result = 0;
    while current != "ZZZ" {
        let next_move = instructions.chars().nth(i).unwrap();
        // print!("From: {current} ");
        match next_move {
            'L' => current = mappings[&current].0.to_string(),
            'R' => current = mappings[&current].1.to_string(),
            _ => panic!("Unexpected instruction"),
        }
        // println!("To: {current} ");
        i += 1;
        if i == len {
            i = 0;
        }
        result += 1;
    }
    println!("Result: {result}");
}

fn part_two() {
    let (instructions, mappings) = parse_data();

    let re_start = Regex::new(r"\w\wA").unwrap();
    let re_end = Regex::new(r"\w\wZ").unwrap();
    let nodes: Vec<&String> = mappings.keys().filter(|x| re_start.is_match(x)).collect();

    let len = instructions.len();
    let result = nodes
        .into_iter()
        .map(|node| {
            let mut i = 0;
            let mut result = 0i64;
            let mut current = node;
            while !re_end.is_match(current) {
                let next_move = instructions.chars().nth(i).unwrap();
                current = match next_move {
                    'L' => &mappings[current].0,
                    'R' => &mappings[current].1,
                    _ => panic!("Unexpected instruction"),
                };
                i += 1;
                if i == len {
                    i = 0;
                }
                result += 1;
            }
            println!("Result: {result}");
            result
        })
        .reduce(num::integer::lcm)
        .unwrap();
    println!("Result is: {result}");
}

fn main() {
    // part_one();
    part_two();
}
