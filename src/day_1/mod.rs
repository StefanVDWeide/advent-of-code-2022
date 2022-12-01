use std::fs;

pub fn solution() {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_1/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    // Use Rust's powerful method chaining to split the string into groups, parse them into i32s and finally sum and sort them.
    let mut groups: Vec<i32> = trimmed_input
        .split("\n\n")
        .map(|x| x.split("\n").collect())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    groups.sort();
    groups.reverse();

    // Print the solution to the console
    println!("The solution to the first question is: {:?}", groups[0]);
    println!(
        "The solution to the second question is: {:?}",
        (groups[0] + groups[1] + groups[2])
    );
}
