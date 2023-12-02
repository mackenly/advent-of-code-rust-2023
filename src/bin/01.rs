advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // total sum of all numbers
    let mut sum: u32 = 0;

    // loop over each line in the input
    for line in input.lines() {
        // get the first number from the line
        let mut number_first: u32 = 0;
        for (_i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                number_first = c.to_digit(10).unwrap();
                break;
            }
        }

        // get the last number from the line
        let mut number_last: u32 = 0;
        for (_i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                number_last = c.to_digit(10).unwrap();
                break;
            }
        }

        // combine the two numbers as a string then parse it as a u32
        let combined: u32 = format!("{}{}", number_first, number_last)
            .parse::<u32>()
            .unwrap();

        // add the combined number to the sum
        sum += combined;
    }

    // return the sum
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
