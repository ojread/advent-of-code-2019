use std::cmp::{max, min};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let input_lines: Vec<_> = input.lines().collect();

    let instructions1 = parse_instructions(input_lines[0]);
    let path1 = calculate_path(instructions1);

    let instructions2 = parse_instructions(input_lines[1]);
    let path2 = calculate_path(instructions2);

    let intersections = find_intersections(&path1, &path2);

    let origin = Point { x: 0, y: 0 };

    let shortest_distance = intersections
        .iter()
        .map(|intersection| get_distance(&origin, intersection))
        .min();

    shortest_distance
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_lines: Vec<_> = input.lines().collect();

    let instructions1 = parse_instructions(input_lines[0]);
    let instructions2 = parse_instructions(input_lines[1]);

    let path1 = calculate_path(instructions1);
    let path2 = calculate_path(instructions2);

    let intersections = find_intersections(&path1, &path2);

    let shortest_steps = intersections
        .iter()
        .map(|intersection| {
            get_path_steps(&path1, intersection) + get_path_steps(&path2, intersection)
        })
        .min();

    shortest_steps
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Instruction {
    dir: Direction,
    length: u32,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .split(",")
        .map(|instruction| {
            let mut chars = instruction.chars();
            let dir = chars.next().unwrap();
            let dir = match dir {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!(""),
            };
            let length: u32 = chars.as_str().parse().expect("Length didn't parse.");
            Instruction { dir, length }
        })
        .collect()
}

fn calculate_path(instructions: Vec<Instruction>) -> Vec<Point> {
    // Track position as we trace the path.
    let mut pos = Point { x: 0, y: 0 };

    // Paths start with the origin point.
    let mut points = vec![Point { x: 0, y: 0 }];

    for instruction in instructions {
        let length = instruction.length as i32;
        match instruction.dir {
            Direction::Up => {
                pos.y += length;
            }
            Direction::Down => {
                pos.y -= length;
            }
            Direction::Left => {
                pos.x -= length;
            }
            Direction::Right => {
                pos.x += length;
            }
        }
        points.push(pos);
    }
    points
}

fn find_intersections(path1: &[Point], path2: &[Point]) -> Vec<Point> {
    let mut intersections: Vec<Point> = Vec::new();

    for i in 0..path1.len() - 1 {
        let a = path1[i];
        let b = path1[i + 1];
        let x1 = min(a.x, b.x);
        let y1 = min(a.y, b.y);
        let x2 = max(a.x, b.x);
        let y2 = max(a.y, b.y);

        for j in 0..path2.len() - 1 {
            let c = path2[j];
            let d = path2[j + 1];
            let x3 = min(c.x, d.x);
            let y3 = min(c.y, d.y);
            let x4 = max(c.x, d.x);
            let y4 = max(c.y, d.y);

            // Lines intersect if their x and y ranges overlap.
            if x1 <= x4 && x3 <= x2 && y1 <= y4 && y3 <= y2 {
                if x1 == x2 && y3 == y4 {
                    intersections.push(Point { x: x1, y: y3 });
                } else if x3 == x4 && y1 == y2 {
                    intersections.push(Point { x: x3, y: y1 });
                }
            }
        }
    }

    intersections
}

fn get_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn get_path_steps(path: &[Point], target: &Point) -> u32 {
    let mut steps: i32 = 0;

    let mut path = path.iter().peekable();

    let mut done = false;

    while let Some(point1) = path.next() {
        if !done {
            if let Some(point2) = path.peek() {
                if !done {
                    // Is the target point between point1 and point2?
                    if (target.x == point1.x
                        && target.x == point2.x
                        && target.y > min(point1.y, point2.y)
                        && target.y < max(point1.y, point2.y))
                        || (target.y == point1.y
                            && target.y == point2.y
                            && target.x > min(point1.x, point2.x)
                            && target.x < max(point1.x, point2.x))
                    {
                        steps += get_distance(point1, target);
                        done = true;
                    } else {
                        // Otherwise add the steps between the two points.
                        steps += get_distance(point1, point2);
                    }
                }
            }
        }
    }

    steps as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4981));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(164012));
    }
}
