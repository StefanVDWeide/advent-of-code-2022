use std::collections::HashMap;
use std::fs;

pub fn solution() {
    // Create a map for the match outcomes and the matching value for part 1
    let outcomes_part_1 = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);

    // Create a map for the match outcomes and the matching value for part 1
    let outcomes_part_2 = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);

    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_2/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    // Split the input by line to create a vector per match
    let matches_groups: Vec<&str> = trimmed_input.lines().collect();

    // Init two vectors for each part of the puzzle.
    let mut part_1_vec = Vec::new();
    let mut part_2_vec = Vec::new();

    // Loop over the match strings and look up the value in the hashmap
    for individual_match in &matches_groups {
        // Push to their respecitve vector
        part_1_vec.push(outcomes_part_1[individual_match]);
        part_2_vec.push(outcomes_part_2[individual_match]);
    }

    // Sum and print the individual vectors to get the solutions
    println!("The part 1 solution is: {}", part_1_vec.iter().sum::<i32>());
    println!("The part 2 solution is: {}", part_2_vec.iter().sum::<i32>());
}
