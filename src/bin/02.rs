advent_of_code::solution!(2);

use advent_of_code::Intcode;

pub fn part_one(input: &str) -> Option<usize> {
    let mut intcode = Intcode::new(input);
    intcode.set_noun(12);
    intcode.set_verb(2);
    let output = intcode.run();
    Some(output)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut intcode = Intcode::new(input);

    let target_output = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            intcode.set_noun(noun);
            intcode.set_verb(verb);
            let output = intcode.run();
            // println!("{}", output);
            if output == target_output {
                // println!("{}, {}", noun, verb);
                // Return the noun and verb as a single number.
                return Some(noun * 100 + verb);
            }
        }
    }

    // If we reached here then the puzzle hasn't been solved.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3101878));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8444));
    }
}
