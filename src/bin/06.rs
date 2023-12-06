advent_of_code::solution!(6);

fn map_tuple(t: (u64,u64)) -> u64 {
    let mut c = 0;
    for hold in 0..(t.0+1) {
        let diff = t.0 - hold;
        let trav = diff * hold;
        if trav > t.1 {
            c += 1;
        }
    }
    println!("{c}");
    c
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let dist = lines.next().unwrap();

    let time_rec = times
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|f| !f.trim().is_empty())
        .map(|a| a.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let dist_rec = dist
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|f| !f.trim().is_empty())
        .map(|a| a.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    std::iter::zip(time_rec, dist_rec)
        .map(map_tuple)
        .reduce(|a,b| a * b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let dist = lines.next().unwrap();

    let time_rec = times
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|f| !f.trim().is_empty())
        .collect::<Vec<&str>>();

    let dist_rec = dist
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .filter(|f| !f.trim().is_empty())
        .collect::<Vec<&str>>();

    let time = time_rec.join("").parse::<u64>().unwrap();
    let dist = dist_rec.join("").parse::<u64>().unwrap();

    Some(map_tuple((time, dist)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(512295));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(36530883));
    }
}
