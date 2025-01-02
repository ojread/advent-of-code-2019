advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let boundaries: Vec<u32> = input
        .split("-")
        .map(|boundary| {
            boundary
                .trim()
                .parse()
                .expect("Boundary couldn't be parsed.")
        })
        .collect();

    // .map(|boundary| boundary.parse().expect("Bounday couldn't be parsed."))
    let lower: u32 = boundaries[0];
    let upper: u32 = boundaries[1];

    let passwords = (lower..=upper)
        .filter(|num| {
            // Password criteria.
            let mut has_double = false;
            let mut has_decreasing = false;

            let password = num.to_string();

            let mut chars = password.chars().peekable();

            while let Some(this) = chars.next() {
                if let Some(next) = chars.peek() {
                    if this == *next {
                        has_double = true;
                    }
                    let d1 = this.to_digit(10).expect("Nah");
                    let d2 = next.to_digit(10).expect("Nah");
                    if d1 > d2 {
                        has_decreasing = true;
                    }
                }
            }

            (has_double && !has_decreasing)
        })
        .count();

    Some(passwords)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
