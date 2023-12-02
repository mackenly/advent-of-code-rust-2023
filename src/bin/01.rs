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
    let spelled_numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;

    for line in input.lines() {
        let first_number = find_number(line, spelled_numbers, false);
        let last_number = find_number(line, spelled_numbers, true);

        sum += format!("{}{}", first_number, last_number)
            .parse::<u32>()
            .unwrap();
    }

    Some(sum)
}

fn find_number(input: &str, spelled_numbers: [&str; 9], reversed: bool) -> u32 {
    let mut input: String = input.to_string();
    if reversed {
        input = input.chars().rev().collect::<String>();
    }
    for (i, c) in input.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
        if c.is_alphabetic() {
            for (j, spelled_number) in spelled_numbers.iter().enumerate() {
                let mut spelled_number: String = spelled_number.to_string();
                if reversed {
                    spelled_number = spelled_number.chars().rev().collect::<String>();
                }
                if input[i..].starts_with(&spelled_number) {
                    return (j + 1) as u32;
                }
            }
        }
    }
    panic!("No number found in input");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
