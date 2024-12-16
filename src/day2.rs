struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn from_str(input: &str) -> Self {
        let levels = input
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        Self { levels }
    }

    fn is_safe(&self) -> bool {
        let is_increasing = self.levels[1] > self.levels[0];
        !self.levels.windows(2).any(|window| {
            let abs_diff = (window[1] - window[0]).abs();
            abs_diff == 0 || abs_diff > 3 || (window[1] > window[0]) != is_increasing
        })
    }

    fn is_safe_dampned(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        (0..self.levels.len()).into_iter().any(|i| {
            let mut levels = self.levels.clone();
            levels.remove(i);
            let report = Report { levels };
            report.is_safe()
        })
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Report> {
    let mut result = Vec::new();
    for line in input.lines() {
        let report = Report::from_str(line);
        result.push(report);
    }

    result
}

#[aoc(day2, part1)]
fn part1(reports: &Vec<Report>) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

#[aoc(day2, part2)]
fn part2(reports: &Vec<Report>) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe_dampned())
        .count()
}
