use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn valid_num_digit(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let vec_pos = vec![
        (i+1, j),
        (i-1, j),
        (i, j+1),
        (i, j-1),
        (i+1, j+1),
        (i+1, j-1),
        (i-1, j+1),
        (i-1, j-1),
    ];

    vec_pos.iter()
        .filter(|pos| {
            let i_j: usize = pos.0.try_into().unwrap_or(1); // dummy default value
            pos.0 >= 0 && pos.0 < grid.len().try_into().unwrap() && pos.1 >= 0 && pos.1 < grid[i_j].len().try_into().unwrap()
        })
        .map(|p| {
            let i_i: usize = p.0.try_into().unwrap();
            let i_j: usize = p.1.try_into().unwrap();
            grid[i_i][i_j]
        })
        .any(|c| !c.is_digit(10) && c != '.')
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut tot = 0;

    for (i, line) in grid.iter().enumerate() {
        let mut curr_idx = 0;

        while curr_idx < line.len() {
            let mut curr_num = 0;
            let mut should_sum = false;

            while curr_idx < line.len() && line[curr_idx].is_digit(10) {
                curr_num = curr_num * 10 + line[curr_idx].to_digit(10).unwrap();
                should_sum = valid_num_digit(&grid, i.try_into().unwrap(), curr_idx.try_into().unwrap()) || should_sum;
                curr_idx += 1;
            }
            if should_sum {
                tot += curr_num;
            }

            curr_idx += 1;
        }
    }

    Some(tot)
}

pub fn get_ratio(grid: &Vec<Vec<char>>, i: i32, j: i32) -> u32 {
    let vec_pos = vec![ // all possible adjacent numbers
        (i+1, j),
        (i-1, j),
        (i, j+1),
        (i, j-1),
        (i+1, j+1),
        (i+1, j-1),
        (i-1, j+1),
        (i-1, j-1),
    ];

    let vs = vec_pos.iter()
        .filter(|pos| {
            let i_j: usize = pos.0.try_into().unwrap_or(1); // dummy default value
            pos.0 >= 0 && pos.0 < grid.len().try_into().unwrap() && pos.1 >= 0 && pos.1 < grid[i_j].len().try_into().unwrap()
        })
        .filter(|p| {
            let i_i: usize = p.0.try_into().unwrap();
            let i_j: usize = p.1.try_into().unwrap();
            grid[i_i][i_j].is_digit(10)
        })
        .map(|p| {
            let mut curr_col: usize = p.1.try_into().unwrap();
            let curr_row: usize = p.0.try_into().unwrap();
            let mut right_digits = Vec::<char>::new();

            while curr_col < grid[curr_row].len() && grid[curr_row][curr_col].is_digit(10) {
                right_digits.push(grid[curr_row][curr_col]);
                curr_col += 1;
            }

            let mut left_digits = Vec::<char>::new();
            curr_col = p.1.try_into().unwrap();

            if curr_col == 0 {
                return right_digits.iter().collect::<String>().parse::<u32>().unwrap();
            }
            curr_col -= 1;

            loop {
                if !grid[curr_row][curr_col].is_digit(10) {
                    break;
                }
                left_digits.push(grid[curr_row][curr_col]);
                if curr_col == 0 {
                    break;
                }
                curr_col -= 1;
            }

            left_digits.reverse();

            left_digits.append(&mut right_digits);


            left_digits.iter().collect::<String>().parse::<u32>().unwrap()
        })
        .filter(|&c| c != 0)
        .collect::<HashSet<u32>>(); // filter unique

    if vs.len() != 2 {
        0
    } else {
        vs.into_iter().reduce(|a,b| a * b).unwrap()
    }

}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut tot = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '*' {
                tot += get_ratio(&grid, i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
    }

    Some(tot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(525181));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
