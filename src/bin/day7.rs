use std::collections::VecDeque;

use advent_of_code_2024::{read_file, Timer};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operators {
    Add,
    Multiply,
    Concatonate,
}

fn run_sequence(sequence: &Vec<Operators>, input: &Vec<isize>) -> isize {
    // Take each operator from the top of the sequence.
    // Carry out the mutation, updating the first entry in the input as appropriate.
    // Return the first value from the input as the final answer.  This approach makes it easy to test as well
    let mut sequence = VecDeque::from(sequence.clone());
    let mut input = input.clone();
    while let Some(operation) = sequence.pop_front() {
        match operation {
            Operators::Add => {
                input[0] += input[1];
            }
            Operators::Multiply => {
                input[0] *= input[1];
            }
            Operators::Concatonate => {
                input[0] = format!("{}{}", input[0], input[1]).parse().unwrap()
            }
        }
        input.remove(1);
    }

    input[0]
}

fn part_two(input: &[(isize, Vec<isize>)]) {
    let timer = Timer::start("Part One".to_owned());
    let operators: Vec<Operators> =
        vec![Operators::Add, Operators::Multiply, Operators::Concatonate];
    let mut sum = 0;
    for (target, values) in input {
        let operations = vec![&operators; values.len() - 1];
        let permutations = operations
            .into_iter()
            .multi_cartesian_product()
            .collect::<VecDeque<_>>();
        for possible_sequence in permutations {
            let total = run_sequence(
                &possible_sequence // This feels ugly, largely caused by type work in tests.
                    .clone()
                    .into_iter()
                    .copied()
                    .collect(),
                values,
            );
            if total == *target {
                sum += total;
                break;
            }
        }
    }
    println!("Answer: {:?}", sum);
    timer.elapsed();
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
            .collect::<VecDeque<_>>();
        for possible_sequence in permutations {
            let total = run_sequence(
                &possible_sequence // This feels ugly, largely caused by type work in tests.
                    .clone()
                    .into_iter()
                    .copied()
                    .collect(),
                values,
            );
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
                    .filter(|&i| !i.is_empty())
                    .map(|z| z.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>(),
            )
        })
        .collect::<Vec<_>>();
    part_one(&input);
    part_two(&input);
    timer.elapsed();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        [Operators::Add, Operators::Add, Operators::Add], 10   // 1 + 2 + 3 + 4 = 10
    )]
    #[case(
        [Operators::Add, Operators::Concatonate, Operators::Add], 37 // 1 + 2 = 3.  3 concatonated with 3 is 33.  33 + 4 = 37
    )]
    fn test_run_squence(#[case] test_sequence: [Operators; 3], #[case] expected: isize) {
        let test_sequence = Vec::from(test_sequence);
        let input = vec![1, 2, 3, 4];
        let result = run_sequence(&test_sequence, &input);
        assert_eq!(result, expected);
    }
}
