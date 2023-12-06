advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Loop over each line in the input
    let maybe_calibrations = input.split("\n").map(|line| {
        // Find and convert the numeric digits
        let digits: Vec<_> = line.matches(char::is_numeric).map(|digit| digit.parse::<u32>().unwrap()).collect();

        // Get the first and last digits
        let first = digits.first()?;
        let last = digits.last()?;

        // Combine using the first digit as the 10s place of the calibration
        return Some(first*10 + last);
    });

    // Add the calibrations from each line together
    return Some(maybe_calibrations.flatten().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    // Create digit token lookup table
    let words: std::collections::HashMap<_,u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ].iter().cloned().collect();

    // Loop over each line in the input
    let maybe_calibrations = input.split("\n").map(|line| {
        // Find and convert the numeric digits by index
        let mut digits: Vec<_> = line.match_indices(char::is_numeric).map(|(index, char)| (index, char.parse::<u32>().unwrap())).collect();

        // Look up the word digits by index and add them to the list
        for word in words.keys() {
            line.match_indices(word).for_each(|(index, _)| digits.push((index, * words.get(word).unwrap())));
        }

        // Sort by index and get the first and last digits of either type
        digits.sort_by(|(index_a, _), (index_b, _)| usize::cmp(index_a, index_b));
        let first = digits.first()?.1;
        let last = digits.last()?.1;

        // Combine using the first digit as the 10s place of the calibration
        return Some(first*10 + last);
    });

    // Add the calibrations from each line together
    return Some(maybe_calibrations.flatten().sum());
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
