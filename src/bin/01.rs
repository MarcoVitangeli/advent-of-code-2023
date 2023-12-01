advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            digits.first().unwrap() * 10 + digits.last().unwrap()
    }).reduce(|a, b| a + b)
}

pub struct IndexedDigit {
    idx: usize,
    value: usize
}

pub fn part_two(input: &str) -> Option<u32> {
    let digit_str = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    input
        .lines()
        .map(|line| {
            let mut digits = Vec::<IndexedDigit>::new();
            for (pos, ds) in digit_str.iter().enumerate() { //O(9)
                for m in line.match_indices(ds) { // O(line) <<<< 100
                    digits.push(IndexedDigit { idx: m.0, value: pos+1 });
                }
            }
            for (pos, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    digits.push(IndexedDigit { idx: pos, value: usize::try_from(c.to_digit(10).unwrap()).unwrap()});
                }
            }
            digits.sort_by(|a, b| a.idx.partial_cmp(&b.idx).unwrap());
            u32::try_from(digits.first().unwrap().value * 10 + digits.last().unwrap().value).unwrap()
        }).reduce(|a, b| a + b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result.unwrap(), 54338);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result.unwrap(), 53389);
    }
}
