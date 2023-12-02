advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    // data vector
    let mut data: Vec<(u32, Vec<(u32, Vec<(&str, u32)>)>)> = Vec::new();

    // valid game ids vector
    let mut valid_game_ids: Vec<u32> = Vec::new();

    // what makes a valid game?
    let red_count: u32 = 12;
    let blue_count: u32 = 14;
    let green_count: u32 = 13;

    for line in input.lines() {
        // line data
        let mut line_data: Vec<(u32, Vec<(&str, u32)>)> = Vec::new();

        // split line on :
        let mut split = line.split(": ");

        // get game id from the first part by removing "Game " from the start
        let game_id: u32 = split.next()?.trim_start_matches("Game ").parse::<u32>().ok()?;

        // trim the second part
        let second_part: &str = split.next()?.trim();

        // split the second part on simicolons
        let sets = second_part.split("; ");

        // for each set
        for (i, set) in sets.enumerate() {
            // split on commas
            let split = set.split(", ");
            let mut store = Vec::new();

            // extract the color name and count from each part (ex: "3 blue" -> ("blue", 3))
            for part in split {
                let mut part = part.split(" ");
                let count = part.next()?.parse::<u32>().ok()?;
                let color = part.next()?;
                store.push((color, count));
            }

            // add the set to the line data
            line_data.push((i as u32, store));
        }

        // add the line data to the data vector
        data.push((game_id, line_data));
    }

    // find the valid game ids
    for (game_id, line_data) in data {
        // count the colors in each set
        let mut valid = true;

        for (_set_id, set) in line_data {
            // get value for each color
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for (color, count) in set {
                match color {
                    "red" => red = count,
                    "blue" => blue = count,
                    "green" => green = count,
                    _ => (),
                }
            }

            // check if the set is valid
            if red > red_count || blue > blue_count || green > green_count {
                valid = false;
                break;
            }
        }

        // if the game is valid, add it to the valid game ids
        if valid {
            valid_game_ids.push(game_id);
        }
    }

    // sum the valid game ids
    let sum: u32 = valid_game_ids.iter().sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // data vector
    let mut data: Vec<(u32, Vec<(u32, Vec<(&str, u32)>)>)> = Vec::new();

    // sum
    let mut sum = 0;

    for line in input.lines() {
        // line data
        let mut line_data: Vec<(u32, Vec<(&str, u32)>)> = Vec::new();

        // split line on :
        let mut split = line.split(": ");

        // get game id from the first part by removing "Game " from the start
        let game_id: u32 = split.next()?.trim_start_matches("Game ").parse::<u32>().ok()?;

        // trim the second part
        let second_part: &str = split.next()?.trim();

        // split the second part on simicolons
        let sets = second_part.split("; ");

        // for each set
        for (i, set) in sets.enumerate() {
            // split on commas
            let split = set.split(", ");
            let mut store = Vec::new();

            // extract the color name and count from each part (ex: "3 blue" -> ("blue", 3))
            for part in split {
                let mut part = part.split(" ");
                let count = part.next()?.parse::<u32>().ok()?;
                let color = part.next()?;
                store.push((color, count));
            }

            // add the set to the line data
            line_data.push((i as u32, store));
        }

        // add the line data to the data vector
        data.push((game_id, line_data));
    }

    // find the valid game ids
    for (game_id, line_data) in data {
        // count the colors in each set
        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for (_set_id, set) in line_data {
            // get value for each color
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;

            for (color, count) in set {
                match color {
                    "red" => red = count,
                    "blue" => blue = count,
                    "green" => green = count,
                    _ => (),
                }
            }

            // check if the set is valid
            if red > max_red {
                max_red = red;
            }

            if blue > max_blue {
                max_blue = blue;
            }

            if green > max_green {
                max_green = green;
            }
        }

        // multiply the max values then add them to sum
        sum += max_red * max_blue * max_green;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
