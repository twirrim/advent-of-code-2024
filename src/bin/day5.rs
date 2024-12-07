use std::ops::Index;

use advent_of_code_2024::{debug_println, read_file, Timer};

// These structs are probably overkill?
#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

#[derive(Debug)]
struct Sequence {
    // Urgh, naming things.
    rules: Vec<Rule>,
}

impl Sequence {
    fn verify(&self, pages: &Vec<u32>) -> bool {
        // This feels inefficient?
        for rule in &self.rules {
            if let Some(before_index) = pages.iter().position(|&x| x == rule.before) {
                if let Some(after_index) = pages.iter().position(|&x| x == rule.after) {
                    if after_index < before_index {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn part_one(sequence: Sequence, to_print: &Vec<String>) {
    let timer = Timer::start("Part one".to_owned());
    let mut sum: u32 = 0;
    for line in to_print {
        let pages: Vec<u32> = line
            .split(',')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        if sequence.verify(&pages) {
            let mid_index = pages.len() / 2;
            sum += pages[mid_index];
        }
    }
    println!("Answer: {:?}", sum);
    timer.elapsed();
}

fn main() {
    let timer = Timer::start("Day 5".to_owned());
    let read_timer = Timer::start("Parsing input".to_owned());
    let input = read_file("./input/day5");
    let (rules, mut to_print): (Vec<_>, Vec<_>) = input.into_iter().partition(|n| n.contains('|'));
    to_print.remove(0); // first entry is a blank entry
    let rules = Sequence {
        rules: rules
            .iter()
            .map(|x| {
                let split: Vec<&str> = x.split('|').collect();
                Rule {
                    before: split[0].parse().unwrap(),
                    after: split[1].parse().unwrap(),
                }
            })
            .collect(),
    };
    read_timer.elapsed();
    part_one(rules, &to_print);
    timer.elapsed();
}
