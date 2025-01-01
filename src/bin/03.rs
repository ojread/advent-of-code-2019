use std::cmp;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    // let wires: Vec<Vec<_>> = input
    //     .lines()
    //     .map(|dirs| {
    //         let mut pos = (0, 0);
    //         dirs.split(",")
    //             .into_iter()
    //             .map(|step| {
    //                 let mut cells: Vec<_> = Vec::new();
    //                 let mut chars = step.chars();
    //                 let dir = chars.next().unwrap();
    //                 let dist: u32 = chars.as_str().parse().expect("Distance didn't parse.");

    //                 let delta = match dir {
    //                     'L' => (-1, 0),
    //                     'R' => (1, 0),
    //                     'D' => (0, -1),
    //                     'U' => (0, 1),
    //                     _ => (0, 0),
    //                 };

    //                 for _i in 1..=dist {
    //                     pos.0 += delta.0;
    //                     pos.1 += delta.1;
    //                     cells.push(format!("{},{}", pos.0, pos.1));
    //                 }

    //                 // println!("{} {} {:?}", dir, dist, cells);
    //                 cells
    //             })
    //             .flatten()
    //             .collect()
    //     })
    //     .collect();

    // println!("wires");

    // let matches: Vec<_> = wires[0]
    //     .clone()
    //     .into_iter()
    //     .filter(|point1| {
    //         wires[1].clone().into_iter().any(|point2| {
    //             println!("{} {}", point1, point2);
    //             **point1 == point2
    //         })
    //     })
    //     .collect();

    // println!("matches");

    // let distances: Vec<i32> = matches
    //     .into_iter()
    //     .map(|point| {
    //         let coords: Vec<i32> = point
    //             .split(",")
    //             .into_iter()
    //             .map(|coord| coord.parse::<i32>().expect("Coord did not parse."))
    //             .collect();
    //         coords[0].abs() + coords[1].abs()
    //     })
    //     .collect();

    // println!("distances");

    // println!("{:?}", distances);

    // let min_distance = distances.into_iter().min();

    // min_distance

    let instructions: Vec<_> = input.lines().collect();

    // let

    let wire1 = Wire::new(instructions[0]);
    let wire2 = Wire::new(instructions[1]);

    // let mut intersections: Vec<Point> = Vec::new();
    // for line1 in &wire1.lines[..] {
    //     for line2 in &wire2.lines[..] {
    //         if line1.intersects(*line2) {
    //             if line1.x1 == line1.x2 && line2.y1 == line2.y2 {
    //                 println!(
    //                     "{},{} {}",
    //                     line1.x1,
    //                     line2.y1,
    //                     line1.x1.abs() + line2.y1.abs()
    //                 );
    //                 intersections.push(Point::new(line1.x1, line2.y1));
    //             } else if line1.y1 == line1.y2 && line2.x1 == line2.x2 {
    //                 println!(
    //                     "{},{} {}",
    //                     line2.x1,
    //                     line1.y1,
    //                     line2.x1.abs() + line1.y1.abs()
    //                 );
    //                 intersections.push(Point::new(line2.x1, line1.y1));
    //             }
    //         }
    //     }
    // }

    let mut intersections: Vec<Point> = Vec::new();

    // let mut wire1_points = wire1.points.iter().peekable();

    // for point1 in wire1_points {
    //     let point2 = &wire1_points.peek();
    //     println!("{:?} {:?}", point1, point2);
    // }

    for i in 0..wire1.points.len() - 1 {
        let a = wire1.points[i];
        let b = wire1.points[i + 1];
        let x1 = cmp::min(a.x, b.x);
        let y1 = cmp::min(a.y, b.y);
        let x2 = cmp::max(a.x, b.x);
        let y2 = cmp::max(a.y, b.y);

        for j in 0..wire2.points.len() - 1 {
            let c = wire2.points[j];
            let d = wire2.points[j + 1];
            let x3 = cmp::min(c.x, d.x);
            let y3 = cmp::min(c.y, d.y);
            let x4 = cmp::max(c.x, d.x);
            let y4 = cmp::max(c.y, d.y);

            // Lines intersect if their x and y ranges overlap.
            if x1 <= x4 && x3 <= x2 && y1 <= y4 && y3 <= y2 {
                if x1 == x2 && y3 == y4 {
                    intersections.push(Point::new(x1, y3));
                } else if x3 == x4 && y1 == y2 {
                    intersections.push(Point::new(x3, y1));
                }
            }
        }
    }

    // println!("{:?}", intersections.clone());

    let distances: Vec<i32> = intersections
        .iter()
        .map(|point| point.x.abs() + point.y.abs())
        .collect();

    let shortest_distance = distances.into_iter().min();

    println!("{:?}", shortest_distance);

    shortest_distance
    // None
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

// fn lines_intersect(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
//     if x1 >= x2 && x1
//     false
// }

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

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Line {
            x1,
            y1,
            x2,
            y2,
            // x1: cmp::min(x1, x2),
            // y1: cmp::min(y1, y2),
            // x2: cmp::max(x1, x2),
            // y2: cmp::max(y1, y2),
        }
    }

    fn intersects(&self, line: Line) -> bool {
        // Lines intersect if their x and y ranges overlap.
        self.x1 <= line.x2 && self.x2 > line.y1 && self.y1 <= line.y2 && self.y2 > line.y1
    }
}

#[derive(Debug)]
struct Wire {
    instructions: Vec<Instruction>,
    lines: Vec<Line>,
    points: Vec<Point>,
}

impl Wire {
    fn new(input: &str) -> Self {
        let instructions: Vec<Instruction> = input
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
            .collect();

        // Track position as we trace the path.
        let mut pos = Point::new(0, 0);
        let mut lines = Vec::new();
        let mut points = Vec::new();

        for instruction in instructions.clone() {
            let length = instruction.length as i32;
            match instruction.dir {
                Direction::Up => {
                    let line = Line::new(pos.x, pos.y, pos.x, pos.y + length);
                    lines.push(line);
                    pos.y += length;
                }
                Direction::Down => {
                    let line = Line::new(pos.x, pos.y, pos.x, pos.y - length);
                    lines.push(line);
                    pos.y -= length;
                }
                Direction::Left => {
                    let line = Line::new(pos.x, pos.y, pos.x - length, pos.y);
                    lines.push(line);
                    pos.x -= length;
                }
                Direction::Right => {
                    let line = Line::new(pos.x, pos.y, pos.x + length, pos.y);
                    lines.push(line);
                    pos.x += length;
                }
            }
            points.push(pos);
        }

        // println!("{:?}", points);

        Self {
            instructions,
            lines,
            points,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(159));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
