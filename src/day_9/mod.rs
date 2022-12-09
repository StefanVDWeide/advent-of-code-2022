use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

pub fn solution() {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_9/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    let commands: Vec<Direction> = trimmed_input
        .lines()
        .map(|line| Direction::from_str(line).unwrap())
        .collect();

    let part_1 = simulate::<2>(&commands);
    let part_2 = simulate::<10>(&commands);
    println!("{part_1}");
    println!("{part_2}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    L(i32),
    R(i32),
    U(i32),
    D(i32),
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s[2..]
            .parse()
            .map_err(|_| format!("cannot parse steps: {}", s))?;

        match &s[..1] {
            "L" => Ok(Direction::L(steps)),
            "R" => Ok(Direction::R(steps)),
            "U" => Ok(Direction::U(steps)),
            "D" => Ok(Direction::D(steps)),
            _ => Err(format!("invalid direction: {}", s)),
        }
    }
}

fn simulate<const N: usize>(input: &Vec<Direction>) -> usize {
    let mut rope = [(0, 0); N];

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for d in input.iter().copied() {
        match d {
            Direction::L(s) => {
                for _ in 0..s {
                    rope[0].1 -= 1;
                    move_rope(&mut rope, &mut visited);
                }
            }

            Direction::R(s) => {
                for _ in 0..s {
                    rope[0].1 += 1;
                    move_rope(&mut rope, &mut visited);
                }
            }

            Direction::U(s) => {
                for _ in 0..s {
                    rope[0].0 += 1;
                    move_rope(&mut rope, &mut visited);
                }
            }

            Direction::D(s) => {
                for _ in 0..s {
                    rope[0].0 -= 1;
                    move_rope(&mut rope, &mut visited);
                }
            }
        }
    }

    visited.len()
}

fn move_rope<const N: usize>(rope: &mut [(i32, i32); N], visited: &mut HashSet<(i32, i32)>) {
    let old_tail = rope[N - 1];

    for h in 0..N - 1 {
        let (hr, hc) = rope[h + 0];
        let (tr, tc) = rope[h + 1];

        let (rx, cx) = move_tail(hr, hc, tr, tc);
        if (rx, cx) == (tr, tc) {
            break;
        }

        rope[h + 1] = (rx, cx);
    }

    if old_tail != rope[N - 1] {
        visited.insert(rope[N - 1]);
    }
}

fn move_tail(hr: i32, hc: i32, mut tr: i32, mut tc: i32) -> (i32, i32) {
    if hc.abs_diff(tc) > 1 || hr.abs_diff(tr) > 1 {
        tr += (hr - tr).signum();
        tc += (hc - tc).signum();
    }

    (tr, tc)
}
