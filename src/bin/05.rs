use std::u64;

    advent_of_code::solution!(5);

struct Map {
    pub _name: String,
    pub ranges: Vec<(u64, u64, u64)> // (source, dest, len)
}

fn get_ranges_and_name(line_buff: Vec<&str>) -> (&str, Vec<(u64, u64, u64)>) {
    let name = line_buff[0];
    let mut ranges: Vec<(u64,u64,u64)> = vec![];
    for i in 1..line_buff.len() {
        let range = line_buff[i]
                .split(" ")
                .map(|f| f.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
        ranges.push((range[1], range[0], range[2]));
    }
    (name, ranges)
}

fn map_apply(seeds: Vec<u64>, m: Map) -> Vec<u64> {
    let mut new_seeds = vec![];
    for s in seeds {
        let mut passed = false;
        for r in &m.ranges {
            if s >= r.0 && s <= r.0 + r.2 - 1 {
                let diff = s - r.0 + r.1;
                new_seeds.push(diff);
                passed = true;
                break;
            }
        }
        if !passed {
            new_seeds.push(s);
        }
    }
    new_seeds
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let seeds_str = lines.nth(0).unwrap();

    let seed_numbers = seeds_str
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let cont: Vec<&str> = lines.collect();
    let mut line_buff: Vec<&str> = vec![];
    let mut maps: Vec<Map> = vec![];

    for l in cont {
        if l.trim().is_empty() && line_buff.len() > 0 {
            let t = get_ranges_and_name(line_buff.clone());
            maps.push(Map {_name: t.0.into(), ranges: t.1});
            line_buff.clear();
        } else if !l.trim().is_empty() {
            line_buff.push(l);
        }
    }
    if line_buff.len() > 0 {
        let t = get_ranges_and_name(line_buff.clone());
        maps.push(Map {_name: t.0.into(), ranges: t.1});
        line_buff.clear();
    }

    let mut final_seeds: Vec<u64> = seed_numbers.clone();
    for m in maps {
        final_seeds = map_apply(final_seeds, m);
    }

    let mut mn = u64::MIN;
    let mut passed = false;

    for s in final_seeds {
        if passed {
            mn = std::cmp::min(s, mn);
        } else {
            mn = s;
            passed = true;
        }
    }

    Some(mn)
}

// too slow, 203s in release mode
pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let seeds_str = lines.nth(0).unwrap();
    let seed_numbers = seeds_str
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut n: u64 = 0;
    let cont: Vec<&str> = lines.collect();

    for sdrange in seed_numbers.chunks(2)    {
        let start = *sdrange.first().unwrap();
        let len = *sdrange.last().unwrap();

        let seed_nums = (start..start+len).collect::<Vec<u64>>();

        let mut line_buff: Vec<&str> = vec![];
        let mut maps: Vec<Map> = vec![];

        for l in &cont {
            if l.trim().is_empty() && line_buff.len() > 0 {
                let t = get_ranges_and_name(line_buff.clone());
                maps.push(Map {_name: t.0.into(), ranges: t.1});
                line_buff.clear();
            } else if !l.trim().is_empty() {
                line_buff.push(l);
            }
        }
        if line_buff.len() > 0 {
            let t = get_ranges_and_name(line_buff.clone());
            maps.push(Map {_name: t.0.into(), ranges: t.1});
            line_buff.clear();
        }

        let mut final_seeds: Vec<u64> = seed_nums.clone();
        for m in maps {
            final_seeds = map_apply(final_seeds, m);
        }

        let mut mn = u64::MIN;
        let mut passed = false;

        for s in final_seeds {
            if passed {
                mn = std::cmp::min(s, mn);
            } else {
                mn = s;
                passed = true;
            }
        }

        if mn < n || n == 0 {
            n = mn
        }
    }
    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(389056265));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(137516820));
    }
}
