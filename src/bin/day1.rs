use advent_of_code_2024::{debug_println, read_file};

fn part_one(left: &Vec<isize>, right: &Vec<isize>) {
    let start = std::time::Instant::now();
    let mut total_difference = 0;
    for index in 0..left.len() {
        let diff = (left[index] - right[index]).abs();
        debug_println!("{diff}");
        total_difference += diff;
    }
    println!("{total_difference}");
    println!("Part one time taken: {:?}", start.elapsed());
}

fn part_two(left: &Vec<isize>, right: &Vec<isize>) {
    let start = std::time::Instant::now();
    let mut score = 0;
    for number in left {
        // This is inefficient, we loop through right for every number in left.  Would *probably* be better to build a hashmap of counts.
        // That said, hashmaps incur a hashing cost, so .. maybe not at the scale of this task?
        let count = right.iter().filter(|&n| n == number).count() as isize;
        score += *number * count;
    }
    println!("{score}");
    println!("Part two time taken: {:?}", start.elapsed());
}

fn main() {
    let start = std::time::Instant::now();
    debug_println!("Starting");
    let mut left: Vec<isize> = vec![];
    let mut right: Vec<isize> = vec![];
    read_file("./input/day1").iter().for_each(|entry| {
        let parts: Vec<&str> = entry.split("   ").collect();
        left.push(parts[0].parse::<isize>().unwrap());
        right.push(parts[1].parse::<isize>().unwrap());
    });
    left.sort();
    right.sort();
    debug_println!("{:?}", left);
    part_one(&left, &right);
    part_two(&left, &right);

    println!("Overall time take: {:?}", start.elapsed());
}
