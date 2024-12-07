#[aoc_generator(day1)]
fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut first, mut second) = (vec![], vec![]);
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let (num1, num2) = (
            split.next().unwrap().parse::<u32>().unwrap(),
            split.next().unwrap().parse::<u32>().unwrap(),
        );
        first.push(num1);
        second.push(num2);
    }

    first.sort();
    second.sort();

    (first, second)
}

#[aoc(day1, part1)]
fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let (first, second) = input;

    first
        .iter()
        .zip(second.iter())
        .map(|(&x, &y)| x.abs_diff(y))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    use std::collections::HashMap;
    let (first, second) = input;

    let freq_map: HashMap<u32, u32> = second.iter().fold(HashMap::new(), |mut acc, &val| {
        *acc.entry(val).or_insert(0) += 1;
        acc
    });

    first
        .iter()
        .map(|&val| {
            let count = freq_map.get(&val).unwrap_or(&0);
            val * count
        })
        .sum()
}
