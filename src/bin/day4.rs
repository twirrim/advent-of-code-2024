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
    let mut count: u32 = 0;
    // x is rows, y is columns
    for x in 0..input.len() {
        for y in 0..input[x].len() {
            if x + 2 < input.len() && y + 2 < input[x].len() {
                // Check the middle for quick elimination
                if input[x + 1][y + 1] == 'A' {
                    // "MAS" left to right
                    // Top left
                    if input[x][y] == 'M'
                    // Bottom Left
                    && input[x+2][y] == 'M'
                    // Bottom Right
                    && input[x+2][y+2] == 'S'
                    // Top Right
                    && input[x][y+2] == 'S'
                    {
                        count += 1;
                    }
                    // "SAM" left to right
                    // Top left
                    if input[x][y] == 'S'
                    // Bottom Left
                    && input[x+2][y] == 'S'
                    // Bottom Right
                    && input[x+2][y+2] == 'M'
                    // Top Right
                    && input[x][y+2] == 'M'
                    {
                        count += 1;
                    }
                    // "MAS" top left, to bottom right. "SAM" bottom left to top right
                    // Top left
                    if input[x][y] == 'M'
                    // Bottom Left
                    && input[x+2][y] == 'S'
                    // Bottom Right
                    && input[x+2][y+2] == 'S'
                    // Top Right
                    && input[x][y+2] == 'M'
                    {
                        count += 1;
                    }
                    // "SAM" top left, to bottom right. "MAS" bottom left to top right
                    // Top left
                    if input[x][y] == 'S'
                    // Bottom Left
                    && input[x+2][y] == 'M'
                    // Bottom Right
                    && input[x+2][y+2] == 'M'
                    // Top Right
                    && input[x][y+2] == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("Answer: {count}");
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
