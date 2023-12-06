advent_of_code::solution!(2);

pub struct Bag {
    red: u32,
    green: u32,
    blue: u32
}

pub struct GameBag {
    idx: u32,
    hands: Vec<Bag>
}

pub fn map_line(line: &str) -> GameBag {
    let idx = line.find(':').unwrap();
    let game_idx = &line[5..idx].parse::<u32>().unwrap();

    let res = &line[idx+1..];
    let mut bags = Vec::<Bag>::new();

    for hand in res.split(";") {
        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        for color_and_count in hand.split(",") {
            let mut pair = color_and_count[1..].split(" ");

            let count = pair.next().unwrap().parse::<u32>().unwrap();
            let color = pair.next().unwrap();

            match color {
                "red" => { red = count }
                "green" => { green = count }
                "blue" => { blue = count }
                _ => unreachable!()
            }
        }
        bags.push(Bag {red, blue, green});
    }

    GameBag { idx: game_idx.clone(), hands: bags }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    input
        .lines()
        .map(map_line)
        .filter(|g| {
           g.hands.iter().all(|h| h.red <= max_red && h.green <= max_green && h.blue <= max_blue)
        })
        .map(|g| g.idx)
        .reduce(|a, b| a + b)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(map_line)
        .map(|g| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            g.hands.iter().for_each(|h| {
                max_red = std::cmp::max(max_red, h.red);
                max_green = std::cmp::max(max_green, h.green);
                max_blue = std::cmp::max(max_blue, h.blue);
            });
            max_green * max_red * max_blue
        })
        .reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2239));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(83435));
    }
}
