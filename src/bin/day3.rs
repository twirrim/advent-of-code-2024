use regex::Regex;

use advent_of_code_2024::read_file;

fn part_one(data: &Vec<String>) {
    let start = std::time::Instant::now();
    let re = Regex::new(r"mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)").unwrap();
    let mut sum: i32 = 0;
    for line in data {
        for (_, [x, y]) in re.captures_iter(line).map(|c| c.extract()) {
            sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap();
        }
    }
    println!("Answer: {sum}");
    println!("Part one time taken: {:?}", start.elapsed());
}

fn main() {
    let start = std::time::Instant::now();
    let input = read_file("./input/day3")
        .into_iter()
        .collect::<Vec<String>>();
    part_one(&input);
    println!("Overall time taken: {:?}", start.elapsed());
}
