use std::fs;

pub fn solution() {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_3/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    // Solve part 1 by going over each line, split the string in 2 parts and finally check if a char is present in both parts
    let part_1: u32 = trimmed_input
        .lines()
        .map(|line| {
            let (compartment_1, compartment_2) = line.split_at(line.len() / 2);

            let common_item = compartment_1
                .chars()
                .find(|&ch| compartment_2.contains(ch))
                .expect("no common item category");

            // Calcultate the priority
            priority(common_item)
        })
        .sum();

    // Solve part two by grabbing 3 lines, comparing the characters and then moving on to the next 3
    let mut part_2 = 0;
    let mut lines = trimmed_input.lines();
    loop {
        let Some(line_1) = lines.next() else { break };
        let line_2 = lines.next().expect("failed to find line 2");
        let line_3 = lines.next().expect("failed to find line 3");

        part_2 += line_1
            .chars()
            .find(|&ch| line_2.contains(ch) && line_3.contains(ch))
            .map(priority)
            .expect("failed to find badge")
    }

    println!("The sum of the priorities of part 1 is: {:?}", part_1);
    println!("The sum of the badges of part 2 is: {:?}", part_2);
}

// Get the priority by subtracting 96 or 38 based on the case of the character
fn priority(common_item: char) -> u32 {
    common_item as u32
        - if common_item.is_ascii_lowercase() {
            96
        } else {
            38
        }
}
