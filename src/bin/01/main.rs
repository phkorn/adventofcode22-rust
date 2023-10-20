use adventofcode22::read_lines_by_line;
use itertools::Itertools;
use std::io::Result;

const TOP_ELVES: usize = 3;

fn main() -> Result<()> {
    let path = "src/bin/01/input.txt";
    let mut sum = 0;
    let mut vector = Vec::new();

    read_lines_by_line(path, |line| {
        match line {
            x if x.is_empty() => {
                vector.push(sum);
                sum = 0;
            }
            x => sum += x.parse::<u32>().unwrap(),
        }
        Ok(())
    })?;
    // 1
    let max_value = vector.iter().copied().max();

    // 2
    vector.sort_by_key(|&w| std::cmp::Reverse(w));
    let max_values = vector.iter().take(TOP_ELVES).cloned().collect::<Vec<u32>>();

    println!("\n### 01 ###");

    match max_value {
        Some(max) => println!("Max: {}", max),
        None => println!("Empty vector"),
    }

    println!("Max #{}: {}", TOP_ELVES, max_values.iter().join(","));
    println!("Sum: {}", max_values.iter().sum::<u32>());

    Ok(())
}
