use std::iter::successors;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let modules = parse_modules(input);
    let fuel_required_for_modules = get_fuel_required_for_modules(&modules);
    Some(fuel_required_for_modules)
}

pub fn part_two(input: &str) -> Option<u32> {
    let modules = parse_modules(input);
    let fuel_required_for_modules_and_fuel = get_fuel_required_for_modules_and_fuel(&modules);
    Some(fuel_required_for_modules_and_fuel)
}

// Parse each line of the input into an integer.
fn parse_modules(input: &str) -> Vec<u32> {
    // Process the input line by line, parse each to an integer and collect into an iterator.
    input
        .lines()
        .map(|line| line.parse().expect("Line was not an integer"))
        .collect()
}

// Add up the fuel required for all modules.
fn get_fuel_required_for_modules(modules: &[u32]) -> u32 {
    // For each module, calculate fuel required and total them all.
    modules
        .iter()
        .map(|mass| get_fuel_requirement(mass).unwrap_or(0))
        .sum()
}

// Add up the fuel required for all modules and fuel.
fn get_fuel_required_for_modules_and_fuel(modules: &[u32]) -> u32 {
    modules
        .iter()
        .map(|mass| {
            successors(Some(*mass), get_fuel_requirement)
                .skip(1)
                .sum::<u32>()
        })
        .sum()
}

// Calculate the fuel required for the given mass.
fn get_fuel_requirement(mass: &u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3380880));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5068454));
    }
}
