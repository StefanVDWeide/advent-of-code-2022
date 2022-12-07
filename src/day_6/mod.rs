use std::collections::HashSet;
use std::fs;

// TODO: Clean up code - split in functions
pub fn solution() {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_6/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    let mut initial_marker_start_of_packet: Vec<&str> =
        trimmed_input[..4].split_terminator("").skip(1).collect();

    let initial_unique_set: HashSet<&str> =
        initial_marker_start_of_packet.clone().into_iter().collect();

    if initial_unique_set.len() == 4 {
        println!(
            "The inital 4 characters form a unique marker: {:?}",
            initial_unique_set
        );
    }

    let individual_characters: Vec<&str> = trimmed_input.split_terminator("").skip(5).collect();

    for index in 0..individual_characters.len() {
        initial_marker_start_of_packet.remove(0);

        initial_marker_start_of_packet.push(individual_characters[index]);

        let loop_unique_set: HashSet<&str> =
            initial_marker_start_of_packet.clone().into_iter().collect();

        if loop_unique_set.len() == 4 {
            println!(
                "These 4 characters form a unique marker: {:?} after {} characters",
                loop_unique_set,
                index + 5
            );
            return;
        }
    }
    // Part 2
    // ####################################################################################
    let mut initial_marker_start_of_message: Vec<&str> =
        trimmed_input[..14].split_terminator("").skip(1).collect();

    let initial_unique_set_messages: HashSet<&str> = initial_marker_start_of_message
        .clone()
        .into_iter()
        .collect();

    if initial_unique_set_messages.len() == 14 {
        println!(
            "The inital 14 characters form a unique marker: {:?}",
            initial_unique_set
        );
    }

    let individual_characters_start_of_message: Vec<&str> =
        trimmed_input.split_terminator("").skip(15).collect();

    for index in 0..individual_characters_start_of_message.len() {
        initial_marker_start_of_message.remove(0);

        initial_marker_start_of_message.push(individual_characters_start_of_message[index]);

        let loop_unique_set: HashSet<&str> = initial_marker_start_of_message
            .clone()
            .into_iter()
            .collect();

        if loop_unique_set.len() == 14 {
            println!(
                "These 14 characters form a unique marker: {:?} after {} characters",
                loop_unique_set,
                index + 15
            );
            return;
        }
    }
}
