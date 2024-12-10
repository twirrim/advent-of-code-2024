use std::collections::HashSet;

use advent_of_code_2024::{debug_println, read_file, Timer};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    facing: Direction,
    off_the_map: bool,
    map: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(map: Vec<Vec<char>>) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut facing = Direction::Up;
        let mut found = false;
        for row_x in 0..map.len() {
            if found {
                break;
            }
            for row_y in 0..map[x].len() {
                found = match map[row_x][row_y] {
                    '^' => {
                        facing = Direction::Up;
                        x = row_x;
                        y = row_y;
                        true
                    }
                    _ => false,
                };
                if found {
                    break;
                }
            }
        }
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert((x, y));
        Guard {
            x,
            y,
            facing,
            off_the_map: false,
            map,
            visited,
        }
    }

    fn step(&mut self) {
        match self.facing {
            Direction::Up => {
                if self.x != 0 && self.map[self.x - 1][self.y] == '#' {
                    self.facing = Direction::Right;
                } else {
                    self.x = self.x - 1;
                    self.visited.insert((self.x, self.y));
                    if self.x == 0 {
                        self.off_the_map = true;
                    }
                }
            }
            Direction::Down => {
                if self.x != self.map.len() - 1 && self.map[self.x + 1][self.y] == '#' {
                    self.facing = Direction::Left;
                } else {
                    self.x = self.x + 1;
                    self.visited.insert((self.x, self.y));
                    if self.x == self.map.len() - 1 {
                        self.off_the_map = true;
                    }
                }
            }
            Direction::Left => {
                if self.y != 0 && self.map[self.x][self.y - 1] == '#' {
                    self.facing = Direction::Up;
                } else {
                    self.y = self.y - 1;
                    self.visited.insert((self.x, self.y));
                    if self.y == 0 {
                        self.off_the_map = true;
                    }
                }
            }
            Direction::Right => {
                if self.y != self.map[self.x].len() - 1 && self.map[self.x][self.y + 1] == '#' {
                    self.facing = Direction::Down;
                } else {
                    self.y = self.y + 1;
                    self.visited.insert((self.x, self.y));
                    if self.y == self.map[self.x].len() - 1 {
                        self.off_the_map = true;
                    }
                }
            }
        }
    }
}

fn part_one(input: &Vec<Vec<char>>) {
    let timer = Timer::start("Part One".to_owned());
    let mut guard = Guard::new(input.clone());
    for row in &guard.map {
        debug_println!("{:?}", row);
    }
    while guard.off_the_map == false {
        debug_println!("Before: {}, {}, {:?}", guard.x, guard.y, guard.facing);
        guard.step();
    }
    println!("Answer: {}", guard.visited.len());
    timer.elapsed();
}

fn main() {
    let timer = Timer::start("Day 6".to_owned());
    let input: Vec<Vec<char>> = read_file("./input/day6")
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<_>>();
    part_one(&input);
    timer.elapsed();
}
