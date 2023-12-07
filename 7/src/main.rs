use std::{cmp::Ordering, collections::HashMap};

static CARD_ORDER: &[char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug)]
struct Bid {
    hand: String,
    bid: i32,
    result: Hand,
}

impl Bid {
    pub fn new(hand: String, bid: i32) -> Bid {
        let result = cards_to_hand(&hand);
        Bid { hand, bid, result }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn tally<T: std::hash::Hash + std::cmp::Eq>(v: &[T]) -> HashMap<&T, i32> {
    let mut res = HashMap::new();
    for el in v {
        let val = match res.get(el) {
            Some(val) => val + 1,
            None => 1,
        };
        res.insert(el, val);
    }
    res
}

fn cards_to_hand(hand: &str) -> Hand {
    let hand_arr: Vec<char> = hand.chars().collect();
    let card_counts = tally(&hand_arr);
    let values: Vec<i32> = card_counts.values().cloned().collect();
    if values.contains(&5) {
        Hand::FiveOfKind
    } else if values.contains(&4) {
        Hand::FourOfKind
    } else if values.contains(&3) && values.contains(&2) {
        Hand::FullHouse
    } else if values.contains(&3) && values.len() == 3 {
        Hand::ThreeOfKind
    } else if values.len() == 3 {
        Hand::TwoPair
    } else if values.contains(&2) {
        Hand::OnePair
    } else {
        Hand::HighCard
    }
}

fn card_val(c: &char) -> usize {
    CARD_ORDER.iter().position(|x| x == c).unwrap()
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    for i in 0..5 {
        let s1 = a.chars().nth(i).unwrap();
        let s2 = b.chars().nth(i).unwrap();
        if s1 != s2 {
            return card_val(&s1).cmp(&card_val(&s2));
        }
    }
    Ordering::Equal
}

fn parse_data() -> Vec<Bid> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            let data: Vec<String> = line.split(' ').map(String::from).collect();
            Bid::new(data[0].clone(), data[1].parse().unwrap())
        })
        .collect()
}

fn part_one() {
    let mut data = parse_data();
    data.sort_by(|a, b| {
        if a.result == b.result {
            compare_hands(&a.hand, &b.hand)
        } else {
            a.result.cmp(&b.result)
        }
    });
    let result: i32 = data
        .iter()
        .enumerate()
        .map(|(x, bid)| (x as i32 + 1) * bid.bid)
        .sum();
    println!("Result is: {result}");
}

fn main() {
    part_one();
}
