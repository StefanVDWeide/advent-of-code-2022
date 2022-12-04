use std::fs;

// TODO: Optimize this solution
pub fn solution() {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string("src/day_4/input.txt").expect("Error during reading the input");

    // Trim the end of the file to ensure no \n will remain to screw you over later
    let trimmed_input = input.trim_end();

    let lines: u32 = trimmed_input
        .lines()
        .map(|line| {
            let (range_1, range_2) = line.split_once(",").unwrap();
            let (start_range_1, end_range_1) = range_1.split_once("-").unwrap();
            let (start_range_2, end_range_2) = range_2.split_once("-").unwrap();

            let a = start_range_1.parse::<i32>().unwrap();
            let b = end_range_1.parse::<i32>().unwrap();
            let c = start_range_2.parse::<i32>().unwrap();
            let d = end_range_2.parse::<i32>().unwrap();

            compare_ranges(a, b, c, d)
        })
        .sum();

    let lines_part_2: u32 = trimmed_input
        .lines()
        .map(|line| {
            let (range_1, range_2) = line.split_once(",").unwrap();
            let (start_range_1, end_range_1) = range_1.split_once("-").unwrap();
            let (start_range_2, end_range_2) = range_2.split_once("-").unwrap();

            let a = start_range_1.parse::<i32>().unwrap();
            let b = end_range_1.parse::<i32>().unwrap();
            let c = start_range_2.parse::<i32>().unwrap();
            let d = end_range_2.parse::<i32>().unwrap();

            find_overlap_in_ranges(a, b, c, d)
        })
        .sum();

    println!("{:?}", lines);
    println!("{:?}", lines_part_2);
}

fn compare_ranges(
    start_range_1: i32,
    end_range_1: i32,
    start_range_2: i32,
    end_range_2: i32,
) -> u32 {
    if start_range_1 >= start_range_2 && end_range_1 <= end_range_2
        || start_range_2 >= start_range_1 && end_range_2 <= end_range_1
    {
        return 1;
    } else {
        return 0;
    }
}

fn find_overlap_in_ranges(
    start_range_1: i32,
    end_range_1: i32,
    start_range_2: i32,
    end_range_2: i32,
) -> u32 {
    for n in start_range_1..=end_range_1 {
        if (start_range_2..=end_range_2).contains(&n) {
            return 1;
        }
    }

    for n in start_range_2..=end_range_2 {
        if (start_range_1..=end_range_1).contains(&n) {
            return 1;
        }
    }

    return 0;
}
