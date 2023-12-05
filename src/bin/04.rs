advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    // loop over each line
    for line in input.lines() {
        // split on ":"
        let mut parts: std::str::Split<'_, &str> = line.split(": ");

        // save second part and trim whitespace
        let numbers: &str = parts.nth(1).unwrap().trim();

        // split on "|"
        let mut numbers: std::str::Split<'_, &str> = numbers.split(" | ");

        // extract both items
        let winning_numbers: Vec<u32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let my_numbers: Vec<u32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        // add points to score
        score += card_processor(&winning_numbers, &my_numbers, "double").unwrap();
    }

    Some(score)
}

fn card_processor(winning_numbers: &Vec<u32>, my_numbers: &Vec<u32>, behavior: &str) -> Option<u32> {
    let mut points = 0;

    // calculate points
    for number in winning_numbers {
        if my_numbers.contains(number) {
            // 1 for the first match, then doubled for each match after
            if points == 0 {
                points = 1;
            } else if behavior == "count" {
                points += 1;
            } else {
                points *= 2;
            }
        }
    }

    //println!("Winning numbers: {:?} My numbers: {:?} Points: {}", winning_numbers, my_numbers, points);

    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matches: Vec<u32> = vec![];

    for line in input.lines() {
        // split on ":"
        let mut parts: std::str::Split<'_, &str> = line.split(": ");

        // save second part and trim whitespace
        let numbers: &str = parts.nth(1).unwrap().trim();

        // split on "|"
        let mut numbers: std::str::Split<'_, &str> = numbers.split(" | ");

        // extract both items
        let winning_numbers: Vec<u32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let my_numbers: Vec<u32> = numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        // add points to score
        matches.push(card_processor(&winning_numbers, &my_numbers, "count").unwrap());
    }

    // calculate score
    let mut cards: Vec<u32> = vec![1; matches.len()];

    // add a zero to the beginning of the vector because cards are 1-indexed
    cards.insert(0, 0);

    // loop over each card
    for (index, num_matches) in matches.iter().enumerate() {
        // calculate the card number
        let card_num: usize = index + 1;

        // add the number of matches to the cards after this one
        for i in (card_num + 1)..=(card_num + *num_matches as usize) {
            cards[i] += cards[card_num];
        }
    }
    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some (13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
