#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Password([u8; 6]);

impl Password {
    // Init digits from a string containing a 6-figure number.
    fn from_str(input: &str) -> Self {
        let int: u32 = input.parse().expect("Input didn't parse.");
        let d0 = ((int / 100_000) % 10) as u8;
        let d1 = ((int / 10_000) % 10) as u8;
        let d2 = ((int / 1_000) % 10) as u8;
        let d3 = ((int / 100) % 10) as u8;
        let d4 = ((int / 10) % 10) as u8;
        let d5 = (int % 10) as u8;

        Self {
            0: [d0, d1, d2, d3, d4, d5],
        }
    }

    // Increment to the next feasible password.
    fn inc(&mut self) {
        for i in (0..6).into_iter().rev() {
            self.0[i] += 1;
            if self.0[i] != 10 {
                return;
            }

            if i == 0 {
                self.0[i] = 0;
            } else {
                self.0[i] = self.0[i - 1];
            }
        }
    }
}

// Does any digit match the next one?
fn has_double_digits(password: &Password) -> bool {
    (0..5).any(|i| password.0[i] == password.0[i + 1])
}

// Are there double digits that aren't part of a triple+ ?
fn has_double_digits_only(password: &Password) -> bool {
    let password = password.0;

    (0..5).any(|i| match i {
        0 => (password[0] == password[1]) && (password[0] != password[2]),
        4 => (password[4] == password[5]) && (password[4] != password[3]),
        n => {
            (password[n] == password[n + 1])
                && (password[n] != password[n - 1])
                && (password[n] != password[n + 2])
        }
    })
}

// Do digits stay the same or increase?
fn does_not_decrease(password: &Password) -> bool {
    (0..5).all(|i| password.0[i] <= password.0[i + 1])
}

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let boundaries: Vec<_> = input.split("-").map(|boundary| boundary.trim()).collect();

    // .map(|boundary| boundary.parse().expect("Bounday couldn't be parsed."))
    let mut test: Password = Password::from_str(boundaries[0]);
    let end: Password = Password::from_str(boundaries[1]);

    let mut count: u32 = 0;

    loop {
        if does_not_decrease(&test) {
            if has_double_digits(&test) {
                count += 1;
            }
        }

        test.inc();

        if test > end {
            break;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let boundaries: Vec<_> = input.split("-").map(|boundary| boundary.trim()).collect();

    // .map(|boundary| boundary.parse().expect("Bounday couldn't be parsed."))
    let mut test: Password = Password::from_str(boundaries[0]);
    let end: Password = Password::from_str(boundaries[1]);

    let mut count: u32 = 0;

    loop {
        if does_not_decrease(&test) {
            if has_double_digits_only(&test) {
                count += 1;
            }
        }

        test.inc();

        if test > end {
            break;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1767));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1192));
    }
}
