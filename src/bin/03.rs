advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    // set to store the start positions of each number
    let mut seen: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    
    // vector to store each line and it's characters
    let mut lines: Vec<Vec<char>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        // add a new vector to the lines vector
        lines.push(Vec::new());
        
        for (j, c) in line.chars().enumerate() {
            // if is space or newline
            if c == ' ' || c == '\n' {
                // skip
                continue;
            }

            // add the character to the lines vector
            lines[i].push(c);

            // if the character is a number
            if c.is_numeric() {
                // if preceding char isn't number
                if j == 0 || !lines[i][j - 1].is_numeric() {
                    // store the position of the number
                    seen.insert((i, j));
                }
            }
        }
    }

    // loop over each seen number
    for (i, j) in seen {
        // if the number is not surrounded by symbols
        if search_around_number(&lines, i, j)? {
            // get the full number
            let number: String = get_full_number(&lines, i, j)?.to_string();
            sum += number.parse::<u32>().unwrap();
        }
    }

    Some(sum)
}

/**
 * Searches around a number to see if it is surrounded by symbols
 * Used for part one
 */
fn search_around_number(lines: &Vec<Vec<char>>, i: usize, j: usize) -> Option<bool> {
    // check right
    if j < lines[i].len() - 1 {
        if is_symbol(lines[i][j + 1]) {
            return Some(true);
        }

        // if the next character is a number recurse
        if lines[i][j + 1].is_ascii_digit() {
            if search_around_number(lines, i, j + 1)? {
                return Some(true);
            }
        }
    }

    // check top right
    if i > 0 && j < lines[i].len() - 1 {
        if is_symbol(lines[i - 1][j + 1]) {
            return Some(true);
        }
    }

    // check bottom right
    if i < lines.len() - 1 && j < lines[i].len() - 1 {
        if is_symbol(lines[i + 1][j + 1]) {
            return Some(true);
        }
    }

    /*if !original {
        return Some(false);
    }*/

    // check above
    if i > 0 {
        if is_symbol(lines[i - 1][j]) {
            return Some(true);
        }
    }

    // check below
    if i < lines.len() - 1 {
        if is_symbol(lines[i + 1][j]) {
            return Some(true);
        }
    }

    // check top left
    if i > 0 && j > 0 {
        if is_symbol(lines[i - 1][j - 1]) {
            return Some(true);
        }
    }

    // check left
    if j > 0 {
        if is_symbol(lines[i][j - 1]) {
            return Some(true);
        }
    }

    // check bottom left
    if i < lines.len() - 1 && j > 0 {
        if is_symbol(lines[i + 1][j - 1]) {
            return Some(true);
        }
    }

    Some(false)
}

/**
 * Checks if a character is a symbol
 * Used for part one
 */
fn is_symbol(c: char) -> bool {
    // a symbol is anything not a number, space, period, newline, or letter
    !c.is_ascii_digit() && c != ' ' && c != '.' && c != '\n' && !c.is_alphabetic()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    // set to store the start positions of each number
    let mut seen: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    
    // vector to store each line and it's characters
    let mut lines: Vec<Vec<char>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        // add a new vector to the lines vector
        lines.push(Vec::new());
        
        for (j, c) in line.chars().enumerate() {
            // if is space or newline
            if c == ' ' || c == '\n' {
                // skip
                continue;
            }

            // add the character to the lines vector
            lines[i].push(c);

            // if the character is a number
            if c == '*' {
                seen.insert((i, j));
            }
        }
    }

    for (i, j) in seen {
        let numbers = find_adjacent_numbers(&lines, i, j).unwrap();

        // if there are two numbers found
        if numbers.len() == 2 {
            // get the numbers
            let product: u32 = numbers[0] * numbers[1];

            // add the product to the sum
            sum += product;
        }
    }

    Some(sum)
}

/**
 * Finds the adjacent numbers to a number
 * Used for part two
 */
fn find_adjacent_numbers(lines: &Vec<Vec<char>>, i: usize, j: usize) -> Option<Vec<u32>> {
    let mut numbers: Vec<u32> = Vec::new();

    // check right
    if j < lines[i].len() - 1 {
        if lines[i][j + 1].is_ascii_digit() {
            numbers.push(get_full_number(lines, i, j + 1)?);
        }
    }

    // check top right
    if i > 0 && j < lines[i].len() - 1 {
        if lines[i - 1][j + 1].is_ascii_digit() {
            numbers.push(get_full_number(lines, i - 1, j + 1)?);
        }
    }

    // check bottom right
    if i < lines.len() - 1 && j < lines[i].len() - 1 {
        if lines[i + 1][j + 1].is_ascii_digit() {
            numbers.push(get_full_number(lines, i + 1, j + 1)?);
        }
    }

    // check above
    if i > 0 {
        if lines[i - 1][j].is_ascii_digit() {
            numbers.push(get_full_number(lines, i - 1, j)?);
        }
    }

    // check below
    if i < lines.len() - 1 {
        if lines[i + 1][j].is_ascii_digit() {
            numbers.push(get_full_number(lines, i + 1, j)?);
        }
    }

    // check top left
    if i > 0 && j > 0 {
        if lines[i - 1][j - 1].is_ascii_digit() {
            numbers.push(get_full_number(lines, i - 1, j - 1)?);
        }
    }

    // check left
    if j > 0 {
        if lines[i][j - 1].is_ascii_digit() {
            numbers.push(get_full_number(lines, i, j - 1)?);
        }
    }

    // check bottom left
    if i < lines.len() - 1 && j > 0 {
        if lines[i + 1][j - 1].is_ascii_digit() {
            numbers.push(get_full_number(lines, i + 1, j - 1)?);
        }
    }

    // handle duplicate numbers
    numbers.sort();
    numbers.dedup();

    Some(numbers)
}

/**
 * Gets the full number from a position
 * Used for part one and two
 */
fn get_full_number(lines: &Vec<Vec<char>>, i: usize, j: usize) -> Option<u32> {
    // get the start and end positions of the number
    let start = lines[i][..j].iter().rposition(|&c| !c.is_ascii_digit()).map_or(0, |p| p + 1);
    let end = lines[i][j..].iter().position(|&c| !c.is_ascii_digit()).map_or(lines[i].len(), |p| j + p);

    let number: String = lines[i][start..end].iter().collect();

    Some(number.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
