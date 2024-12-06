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

fn part_two(data: &Vec<String>) {
    let start = std::time::Instant::now();
    let re =
        Regex::new(r"(mul\((?P<X>\d{1,3}),(?P<Y>\d{1,3})\)|(?P<Do>do\(\))|(?P<Dont>don't\(\)))")
            .unwrap();
    let mut sum: i32 = 0;
    let mut enabled = true; // Flipped by "do" and "don't", starts enabled.
    for line in data {
        for caps in re.captures_iter(line) {
            if let Some(x) = caps.name("X") {
                // X will always be with Y
                let y = caps.name("Y").unwrap();
                if enabled {
                    sum += x.as_str().parse::<i32>().unwrap() * y.as_str().parse::<i32>().unwrap();
                };
            } else if let Some(_) = caps.name("Do") {
                enabled = true;
            } else if let Some(_) = caps.name("Dont") {
                enabled = false;
            }
        }
    }
    println!("Answer: {sum}");
    println!("Part two time taken: {:?}", start.elapsed());
}

fn main() {
    let start = std::time::Instant::now();
    let input = read_file("./input/day3")
        .into_iter()
        .collect::<Vec<String>>();
    part_one(&input);
    part_two(&input);
    println!("Overall time taken: {:?}", start.elapsed());
}
