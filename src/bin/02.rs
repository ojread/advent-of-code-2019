advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let mut state: Vec<usize> = input
        .split(",")
        .map(|int| int.trim().parse::<usize>().unwrap())
        .collect();

    let mut running: bool = true;
    let mut index: usize = 0;

    while running {
        let opcode: usize = state[index];

        match opcode {
            1 => {
                let a: usize = state[state[index + 1]];
                let b: usize = state[state[index + 2]];
                let c: usize = state[index + 3];
                state[c] = a + b;
            }
            2 => {
                let a: usize = state[state[index + 1]];
                let b: usize = state[state[index + 2]];
                let c: usize = state[index + 3];
                state[c] = a * b;
            }
            99 => {
                running = false;
            }
            _ => {}
        }
        index += 4;
    }

    println!("{:?}", state);

    Some(state[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn add(a: u32, b: u32) -> u32 {
    a + b
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
        assert_eq!(result, None);
    }
}
