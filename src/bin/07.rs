use std::cmp::Ord;
advent_of_code::solution!(7);

// final solution inspiration from https://github.com/Coletronix/advent-of-code-rust/blob/main/src/bin/07.rs
// I don't have a future as a poker player :(

// hashmap to count the number of each card kind
type CardCount = std::collections::HashMap<char, u32>;

// function to count the number of each card kind
fn count_cards(cards: &str) -> CardCount {
    let mut card_count: CardCount = std::collections::HashMap::new();
    for card in cards.chars() {
        let count = card_count.entry(card).or_insert(0);
        *count += 1;
    }
    card_count
}

// function to determine the hand type
fn determine_hand_type(cards: &str) -> u32 {
    let card_count: CardCount = count_cards(cards);

    // sort card count
    let mut card_count: Vec<(&char, &u32)> = card_count.iter().collect();
    card_count.sort_by(|a, b| b.1.cmp(a.1));

    // determine hand type
    let hand_type: u32 = match card_count[0].1 {
        5 => 6, // five of a kind
        4 => 5, // four of a kind
        3 => {
            if card_count[1].1 == &2 {
                4 // full house
            } else {
                3 // three of a kind
            }
        }
        2 => {
            if card_count[1].1 == &2 {
                2 // two pair
            } else {
                1 // one pair
            }
        }
        _ => 0, // high card
    };
    hand_type
}

// not 251141024, it's too high
// not 249399453, it's too low
// not 250818195, it's too low
// answer: 251121738
pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u32 = 0;

    let card_map = [
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<char, u32>>();


    let hands = lines.iter().map(|line| {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();
        let hand_type = determine_hand_type(hand);

        (hand, bid, hand_type)
    });

    let mut sorted_hands = hands.collect::<Vec<(&str, u32, u32)>>();
    sorted_hands.sort_by(|(hand1, _, hand1_type), (hand2, _, hand2_type)| {
        // sort by type
        match hand1_type.cmp(hand2_type) {
            std::cmp::Ordering::Equal => (),
            others => return others,
        }
        // sort by kind
        for (a, b) in hand1.chars().zip(hand2.chars()) {
            let first_card_valued = card_map.get(&a).unwrap();
            let second_card_valued = card_map.get(&b).unwrap();
            match first_card_valued.cmp(second_card_valued) {
                std::cmp::Ordering::Equal => (),
                others => return others,
            }
        }
        // should never get here
        std::cmp::Ordering::Equal
    });

    for (index, (_hand, bid, _)) in sorted_hands.iter().enumerate() {
        sum += (index as u32 + 1) * bid;
    }

    Some(sum)
}

// answer: 251421071
pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum: u32 = 0;

    let card_map = [
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 1),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]
    .iter()
    .cloned()
    .collect::<std::collections::HashMap<char, u32>>();


    let hands = lines.iter().map(|line| {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();
        
        let mut max_char = ' ';
        let mut max_count = 0;
        let mut current_char = ' ';
        let mut current_count = 0;
        let mut hand_chars: Vec<char> = hand.chars().collect();
        hand_chars.sort();

        for &c in hand_chars.iter() {
            if c != 'J' {
                if c == current_char {
                    current_count += 1;
                } else {
                    current_char = c;
                    current_count = 1;
                }

                if current_count > max_count {
                    max_char = current_char;
                    max_count = current_count;
                }
            }
        }

        let substituted_hand_chars = hand.chars().map(|c| if c == 'J' { max_char } else { c });

        let hand_type = determine_hand_type(&substituted_hand_chars.collect::<String>());
        (hand, bid, hand_type)
    });

    let mut sorted_hands = hands.collect::<Vec<(&str, u32, u32)>>();
    sorted_hands.sort_by(|(hand1, _, hand1_type), (hand2, _, hand2_type)| {
        // sort by type
        match hand1_type.cmp(hand2_type) {
            std::cmp::Ordering::Equal => (),
            others => return others,
        }
        // sort by kind
        for (a, b) in hand1.chars().zip(hand2.chars()) {
            let first_card_valued = card_map.get(&a).unwrap();
            let second_card_valued = card_map.get(&b).unwrap();
            match first_card_valued.cmp(second_card_valued) {
                std::cmp::Ordering::Equal => (),
                others => return others,
            }
        }
        // should never get here
        std::cmp::Ordering::Equal
    });

    for (index, (_hand, bid, _)) in sorted_hands.iter().enumerate() {
        sum += (index as u32 + 1) * bid;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_one_alt() {
        let result = part_one("9KKK7 890\nJAA48 80\n755Q5 520\n4JQQJ 223\n997T9 405\n8A888 575\nK369T 730\nA5565 847");
        assert_eq!(result, Some(21533));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
