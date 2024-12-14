use advent_of_code_2024::{read_file, Timer};

use itertools::Itertools;

#[derive(Debug)]
enum Operators {
    Add,
    Multiply,
}

fn part_one(input: &[(isize, Vec<isize>)]) {
    let timer = Timer::start("Part One".to_owned());
    let operators: Vec<Operators> = vec![Operators::Add, Operators::Multiply];
    let mut sum = 0;
    for (target, values) in input {
        let operations = vec![&operators; values.len() - 1];
        let permutations = operations
            .into_iter()
            .multi_cartesian_product()
            .collect::<Vec<_>>();
        for possible_sequence in permutations {
            let mut total = values[0];
            for index in 1..values.len() {
                match possible_sequence[index - 1] {
                    Operators::Add => total += values[index],
                    Operators::Multiply => total *= values[index],
                }
            }
            if total == *target {
                sum += total;
                break;
            }
        }
    }
    println!("Answer: {:?}", sum);
    timer.elapsed();
}

fn main() {
    let timer = Timer::start("Day 7".to_owned());
    let input: Vec<(isize, Vec<isize>)> = read_file("./input/day7")
        .iter()
        .map(|x| {
            let y: Vec<&str> = x.split(":").collect();
            (
                y[0].parse::<isize>().unwrap(),
                y[1].split(" ")
                    .filter(|&i| i != "")
                    .map(|z| z.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>(),
            )
        })
        .collect::<Vec<_>>();
    part_one(&input);
    timer.elapsed();
}
