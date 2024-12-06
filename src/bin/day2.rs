use advent_of_code_2024::{debug_println, read_file};

#[derive(Debug)]
enum ReportState {
    Safe,
    Unsafe,
}

fn eval_report(report: &Vec<isize>) -> ReportState {
    debug_println!("{:?}", report);
    let increasing: bool = report[0] < report[1];
    debug_println!("Increasing? {:?}", increasing);
    for i in 0..(report.len() - 1) {
        let diff = report[i + 1] - report[i];
        debug_println!("diff: {diff}");
        if diff == 0 {
            debug_println!("{} - {} == 0: Unsafe", report[i + 1], report[i]);
            return ReportState::Unsafe;
        }
        if diff < 0 && increasing {
            debug_println!("Decreasing, when it should be increasing. Unsafe");
            return ReportState::Unsafe;
        }
        if diff > 0 && !increasing {
            debug_println!("Increasing, when it should be decreasing. Unsafe");
            return ReportState::Unsafe;
        }
        // At this stage we know they're increasing or decreasing, at least by one, so just check
        // if outside the upper bound
        if diff > 3 || diff < -3 {
            debug_println!("Unsafe");
            return ReportState::Unsafe;
        }
    }
    debug_println!("Safe");
    return ReportState::Safe;
}

fn variant_eval(report: &Vec<isize>) -> ReportState {
    for index in 0..report.len() {
        let mut mutated = report.clone();
        mutated.remove(index);
        match eval_report(&mutated) {
            ReportState::Safe => return ReportState::Safe,
            ReportState::Unsafe => (),
        }
    }
    return ReportState::Unsafe;
}

fn part_one(reports: &Vec<Vec<isize>>) {
    let start = std::time::Instant::now();
    let mut safe = 0;
    for report in reports {
        match eval_report(&report) {
            ReportState::Safe => safe += 1,
            ReportState::Unsafe => (),
        };
    }
    println!("Answer: {safe}");
    println!("Part one time taken: {:?}", start.elapsed());
}

fn part_two(reports: &Vec<Vec<isize>>) {
    let start = std::time::Instant::now();
    let mut safe = 0;
    for report in reports {
        match eval_report(&report) {
            ReportState::Safe => safe += 1,
            ReportState::Unsafe => match variant_eval(&report) {
                ReportState::Safe => safe += 1,
                ReportState::Unsafe => (),
            },
        };
    }
    println!("Answer: {safe}");
    println!("Part two time taken: {:?}", start.elapsed());
}

fn main() {
    let start = std::time::Instant::now();
    debug_println!("Starting");
    let mut reports: Vec<Vec<isize>> = vec![];
    read_file("./input/day2").iter().for_each(|entry| {
        let parts: Vec<isize> = entry
            .split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        reports.push(parts);
    });
    println!("Time taken to read and parse input: {:?}", start.elapsed());
    debug_println!("{:?}", reports);
    part_one(&reports);
    part_two(&reports);

    println!("Overall time take: {:?}", start.elapsed());
}
