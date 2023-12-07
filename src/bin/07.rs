use std::collections::HashMap;

advent_of_code::solution!(7);

fn get_type_weight(cards: &Vec<char>) -> u64 {
    let mut m = HashMap::<char, u32>::new();

    for c in cards {
        m.insert(*c, *m.get(c).unwrap_or(&0)+1);
    }

    match m.len() {
        1 => 7, // five of kind
        2 => {
            if m.iter().any(|f| *f.1 == 4 || *f.1 == 1) {
                6 // four of kind
            } else {
                5 // full house
            }
        },
        3 => {
            if m.iter().any(|f| *f.1 == 3) {
                4 // three of kind
            } else {
                3 // two pair
            }
        },
        4 => 2, //one pair
        5 => 1,
        _ => unreachable!(),
    }
}

fn get_type_weight_bis(cards: &Vec<char>) -> u64 {
    let mut m = HashMap::<char, u32>::new();

    for c in cards {
        m.insert(*c, *m.get(c).unwrap_or(&0)+1);
    }

    match m.len() {
        1 => 7, // five of kind
        2 => {
            if m.iter().any(|f| *f.1 == 4 || *f.1 == 1) {
                // four of kind
                if *m.get(&'J').unwrap_or(&0) != 0 {
                    7
                } else {
                    6
                }
            } else {
                if *m.get(&'J').unwrap_or(&0) != 0 {
                    7
                } else {
                    5
                }
            }
        },
        3 => {
            if m.iter().any(|f| *f.1 == 3) {
                //4 // three of kind
                let jokers = *m.get(&'J').unwrap_or(&0);
                match jokers {
                    0 => 4,
                    1 | 3 => 6,
                    _ => unreachable!()
                }
            } else {
                //3 // two pair
                let jokers = *m.get(&'J').unwrap_or(&0);
                match jokers {
                    0 => 3,
                    1 => 5,
                    2 => 6,
                    _ => unreachable!()
                }
            }
        },
        4 => {
            let jokers = *m.get(&'J').unwrap_or(&0);
            //2 //one pair
            match jokers {
                0 => 2,
                1 | 2 => 4,
                _ => unreachable!()
            }
        },
        5 => {
            let jokers = *m.get(&'J').unwrap_or(&0);
            match jokers {
                0 => 1,
                1 => 2,
                _ => unreachable!()
            }
        },
        _ => unreachable!(),
    }
}

fn get_value(c: char) -> u64 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap().into()
    } else {
        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!()
        }
    }
}

fn get_value_bis(c: char) -> u64 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap().into()
    } else {
        match c {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!()
        }
    }
}

struct Hand {
    pub weight: u64,
    pub card_values: Vec<u64>,
    pub bid: u64,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| {
            let mut sp = l.split(" ");
            let cards = sp.next().unwrap().chars().collect::<Vec<char>>();
            let bid = sp.next().unwrap().parse::<u64>().unwrap();

            let ttype = get_type_weight(&cards);

            Hand {
                weight: ttype,
                card_values: cards.iter().map(|c| get_value(*c)).collect(),
                bid: bid,
            }
        }).collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        if a.weight != b.weight {
            a.weight.partial_cmp(&b.weight).unwrap()
        } else {
            let col = std::iter::zip(a.card_values.clone(), b.card_values.clone());
            for pair in  col {
                if pair.0 != pair.1 {
                    return pair.0.partial_cmp(&pair.1).unwrap();
                }
            }
            std::cmp::Ordering::Equal
        }
    });

    let mut res = 0;
    for (i, h) in hands.iter().enumerate() {
        res += (i+1) as u64 * h.bid;
    }
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| {
            let mut sp = l.split(" ");
            let cards = sp.next().unwrap().chars().collect::<Vec<char>>();
            let bid = sp.next().unwrap().parse::<u64>().unwrap();

            let ttype = get_type_weight_bis(&cards);

            Hand {
                weight: ttype,
                card_values: cards.iter().map(|c| get_value_bis(*c)).collect(),
                bid: bid,
            }
        }).collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        if a.weight != b.weight {
            a.weight.partial_cmp(&b.weight).unwrap()
        } else {
            let col = std::iter::zip(a.card_values.clone(), b.card_values.clone());
            for pair in  col {
                if pair.0 != pair.1 {
                    return pair.0.partial_cmp(&pair.1).unwrap();
                }
            }
            std::cmp::Ordering::Equal
        }
    });

    let mut res = 0;
    for (i, h) in hands.iter().enumerate() {
        res += (i+1) as u64 * h.bid;
    }
    Some(res as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(250957639));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(251515496));
    }
}
