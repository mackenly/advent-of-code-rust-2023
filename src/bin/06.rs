advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let [times, distances] = parse_input_for_one(input);
    let mut possible_outcome_count: Vec<u32> = Vec::new();

    // loop over races and save the possible times and distances
    for i in 0..times.len() {
        let outcomes = compute_race_outcomes(times[i] as u64, distances[i] as u64, 1);
        possible_outcome_count.push(outcomes.len() as u32);
    }

    // multiply the possible outcomes together
    Some(possible_outcome_count.iter().product())
}

// two items, times and record distances
fn parse_input_for_one(input: &str) ->  [Vec<u32>; 2] {
    // first line is the times, second line is the distances
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    // parse the times by splitting on colon, then split the second part on space removing any empty
    let times: Vec<u32> = times
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect();

    // parse the distances by splitting on space, then split the second part on comma removing any empty
    let distances: Vec<u32> = distances
        .split(' ')
        .flat_map(|s| s.split(','))
        .filter_map(|s| s.parse().ok())
        .collect();

    [times, distances]
}

// possible times and their distances that break the record distance
fn compute_race_outcomes(time_limit: u64, record_distance: u64, speed_per_millisecond: u64) -> Vec<(u64, u64)> {
    let mut outcomes = Vec::new();
    
    for time in 0..time_limit {
        let speed = time * speed_per_millisecond;
        let distance = (time_limit - time) * speed;
        if distance > record_distance {
            outcomes.push((time, distance));
        }
    }

    outcomes
}

pub fn part_two(input: &str) -> Option<u32> {
    // first line is the times, second line is the distances
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    // parse the times by splitting on colon, then remove spaces and parse to u32
    let time: u64 = times
        .split(':')
        .nth(1)
        .unwrap().replace(" ", "").parse().unwrap();

    let distance: u64 = distances
        .split(':')
        .nth(1)
        .unwrap().replace(" ", "").parse().unwrap();

    let outcomes = compute_race_outcomes(time, distance, 1);

    Some(outcomes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
