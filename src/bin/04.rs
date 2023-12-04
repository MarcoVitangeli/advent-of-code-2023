advent_of_code::solution!(4);

fn map_line(line: &str) -> Vec<u32> {
    let res = line.split(":").last().unwrap();
    let mut cards = res.split("|");

    let winning = cards.next().unwrap().trim().split(" ").filter(|f| !f.trim().is_empty()).map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let have = cards.next().unwrap().trim().split(" ").filter(|f| !f.trim().is_empty()).map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    have
        .into_iter()
        .filter(|num| winning.contains(num))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(map_line).map(|winning| {
        match winning.len() {
            0 => 0,
            1 => 1,
            x => {
                let mut res = 1;
                for _ in 1..x {
                    res *= 2;
                }
                res
            }
        }
    })
    .reduce(|a,b| a + b)
}

#[derive(Clone, Copy)]
struct Card {
    idx: u32,
    count_cards: u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let card_tuples = input
        .lines()
        .map(|line| {
            let mut it = line.split(":");
            let card_id = it.nth(0).unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
            let res = it.last().unwrap();
            let mut cards = res.split("|");

            let winning = cards.next().unwrap().trim().split(" ").filter(|f| !f.trim().is_empty()).map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();

            let have = cards.next().unwrap().trim().split(" ").filter(|f| !f.trim().is_empty()).map(|f| f.parse::<u32>().unwrap()).collect::<Vec<u32>>();

            let c = have
                .into_iter()
                .filter(|num| winning.contains(num))
                .count();
            Card {idx: card_id, count_cards: usize::try_into(c).unwrap() }
        })
        .collect::<Vec<Card>>();

    let mut curr_cards: Vec<Card> = card_tuples.clone();
    let mut res: u32 = 0;

    loop {
        let mut next_cards: Vec<Card> = vec![];

        for card in &curr_cards {
            res += 1;
            for i in 1..(card.count_cards+1) {
                next_cards.push(
                    *card_tuples.iter().find(|c| c.idx == card.idx + i).unwrap()
                )
            }
        }

        if next_cards.is_empty() {
            break;
        }

        curr_cards = next_cards.clone();
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(21138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(7185540));
    }
}
