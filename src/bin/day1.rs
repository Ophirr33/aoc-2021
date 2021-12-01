use aoc_2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let depths: Vec<usize> = read_input(1)?
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{}", count_increases(&depths));
    println!("{}", count_increases(&three_window(&depths)));
    Ok(())
}

fn three_window(depths: &Vec<usize>) -> Vec<usize> {
    depths
        .iter()
        .zip(depths[1..].iter())
        .zip(depths[2..].iter())
        .map(|((&n, &m), &o)| n + m + o)
        .collect()
}

fn count_increases(depths: &Vec<usize>) -> usize {
    depths[1..]
        .iter()
        .zip(depths.iter())
        .filter(|(&depth, &prior_depth)| depth > prior_depth)
        .count()
}
