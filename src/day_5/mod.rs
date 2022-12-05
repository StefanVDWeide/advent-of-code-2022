use std::fs;

// TODO: Also read the inital state from the input file instead of setting it manually
pub fn solution() {
    let mut part_1_initial_state = vec![
        vec!["S", "Z", "P", "D", "L", "B", "F", "C"],
        vec!["N", "V", "G", "P", "H", "W", "B"],
        vec!["F", "W", "B", "J", "G"],
        vec!["G", "J", "N", "F", "L", "W", "C", "S"],
        vec!["W", "J", "L", "T", "P", "M", "S", "H"],
        vec!["B", "C", "W", "G", "F", "S"],
        vec!["H", "T", "P", "M", "Q", "B", "W"],
        vec!["F", "S", "W", "T"],
        vec!["N", "C", "R"],
    ];

    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_5/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    // Transform the file input into a vector of vectors of integers
    let instructions: Vec<Vec<i32>> = trimmed_input
        .lines()
        .skip(10)
        .map(|line| line.split(" ").flat_map(|moves| moves.parse()).collect())
        .collect();

    // Loop over the instructions and move the crates around
    for instruction in instructions {
        let amount_to_be_moved: usize = instruction[0].try_into().unwrap();
        let stack_from_to_be_moved: usize = instruction[1].try_into().unwrap();
        let stack_to_be_moved_into: usize = instruction[2].try_into().unwrap();

        let from_move_range = part_1_initial_state[stack_from_to_be_moved - 1].len()
            - amount_to_be_moved
            ..part_1_initial_state[stack_from_to_be_moved - 1].len();

        // Remove the rev in order to solve part 2
        let mut crates_to_be_moved = part_1_initial_state[stack_from_to_be_moved - 1]
            .drain(from_move_range)
            .as_slice()
            .to_vec()
            .into_iter()
            .rev()
            .collect();

        part_1_initial_state[stack_to_be_moved_into - 1].append(&mut crates_to_be_moved);
    }

    for mut stack in part_1_initial_state {
        let top_crate = stack.pop().unwrap();
        println!("{}", top_crate);
    }
}
