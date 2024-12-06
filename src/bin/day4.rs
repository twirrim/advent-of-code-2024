use advent_of_code_2024::read_file;

fn part_one(input: &Vec<Vec<char>>) {
    let start = std::time::Instant::now();
    let mut count: u32 = 0;
    // x is rows, y is columns
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if y + 3 < input[x].len() {
                // Check horizontal
                if input[x][y] == 'X' // Left to Right
                    && input[x][y + 1] == 'M'
                    && input[x][y + 2] == 'A'
                    && input[x][y + 3] == 'S'
                {
                    count += 1;
                } else if input[x][y] == 'S' // Right to Left
                    && input[x][y + 1] == 'A'
                    && input[x][y + 2] == 'M'
                    && input[x][y + 3] == 'X'
                {
                    count += 1;
                }
            }
            // Check vertical
            if x + 3 < input.len() {
                if input[x][y] == 'X' // Top to Bottom
                    && input[x + 1][y] == 'M'
                    && input[x + 2][y] == 'A'
                    && input[x + 3][y] == 'S'
                {
                    count += 1;
                } else if input[x][y] == 'S' // Bottom to Top
                    && input[x + 1][y] == 'A'
                    && input[x + 2][y] == 'M'
                    && input[x + 3][y] == 'X'
                {
                    count += 1;
                }
            }
            // Check Diagonal, top left, bottom right
            if x + 3 < input.len() && y + 3 < input[x].len() {
                if input[x][y] == 'X' // Forwards
                    && input[x + 1][y + 1] == 'M'
                    && input[x + 2][y + 2] == 'A'
                    && input[x + 3][y + 3] == 'S'
                {
                    count += 1;
                } else if input[x][y] == 'S' // Backwards
                && input[x + 1][y + 1] == 'A'
                && input[x + 2][y + 2] == 'M'
                && input[x + 3][y + 3] == 'X'
                {
                    count += 1;
                }
            }
            // Check Diagonal, top right, bottom left
            if x >= 3 && y + 3 < input[x].len() {
                if input[x][y] == 'X' // Forwards
                    && input[x - 1][y + 1] == 'M'
                    && input[x - 2][y + 2] == 'A'
                    && input[x - 3][y + 3] == 'S'
                {
                    count += 1;
                } else if input[x][y] == 'S' // Backwards
                    && input[x - 1][y + 1] == 'A'
                    && input[x - 2][y + 2] == 'M'
                    && input[x - 3][y + 3] == 'X'
                {
                    count += 1;
                }
            }
        }
    }
    println!("Answer: {count}");
    println!("Part one time taken: {:?}", start.elapsed());
}

fn part_two(input: &Vec<Vec<char>>) {
    let start = std::time::Instant::now();
    println!("Part two time taken: {:?}", start.elapsed());
}

fn main() {
    let start = std::time::Instant::now();
    let input = read_file("./input/day4")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();
    part_one(&input);
    part_two(&input);
    println!("Overall time taken: {:?}", start.elapsed());
}
