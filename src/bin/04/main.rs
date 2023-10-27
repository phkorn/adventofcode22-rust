use adventofcode22::read_lines_by_line;
use std::{io::Result, ops::RangeInclusive};

trait RangeExtension<T> {
    fn intersects(&self, range: &RangeInclusive<T>) -> bool;
    fn overlaps(&self, range: &RangeInclusive<T>) -> bool;
}

impl<T: std::cmp::PartialOrd> RangeExtension<T> for RangeInclusive<T> {
    fn intersects(&self, range: &RangeInclusive<T>) -> bool {
        range.start() <= self.start() && range.end() >= self.end()
    }
    fn overlaps(&self, range: &RangeInclusive<T>) -> bool {
        range.start() <= self.end() && range.start() >= self.start()
    }
}

fn first() -> Result<()> {
    let path = "src/bin/04/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line| {
        let inputs: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = inputs.first().unwrap().split('-').collect();
        let second: Vec<&str> = inputs.last().unwrap().split('-').collect();

        let first_range = RangeInclusive::new(
            first.first().unwrap().parse::<i32>().unwrap(),
            first.last().unwrap().parse::<i32>().unwrap(),
        );
        let second_range = RangeInclusive::new(
            second.first().unwrap().parse::<i32>().unwrap(),
            second.last().unwrap().parse::<i32>().unwrap(),
        );

        print!("{:?} ,  {:?} ", first_range, second_range);
        if first_range.intersects(&second_range) || second_range.intersects(&first_range) {
            sum += 1;
            println!("insect");
        } else {
            println!("do not intersect");
        }
        Ok(())
    })?;

    println!("Sum: {}", sum);
    Ok(())
}

fn second() -> Result<()> {
    let path = "src/bin/04/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line| {
        let inputs: Vec<&str> = line.split(',').collect();
        let first: Vec<&str> = inputs.first().unwrap().split('-').collect();
        let second: Vec<&str> = inputs.last().unwrap().split('-').collect();

        let first_range = RangeInclusive::new(
            first.first().unwrap().parse::<i32>().unwrap(),
            first.last().unwrap().parse::<i32>().unwrap(),
        );
        let second_range = RangeInclusive::new(
            second.first().unwrap().parse::<i32>().unwrap(),
            second.last().unwrap().parse::<i32>().unwrap(),
        );

        print!("{:?} ,  {:?} ", first_range, second_range);
        if first_range.overlaps(&second_range) || second_range.overlaps(&first_range) {
            sum += 1;
            println!("overlaps");
        } else {
            println!("do not overlap");
        }
        Ok(())
    })?;

    println!("Sum: {}", sum);
    Ok(())
}

fn main() -> Result<()> {
    println!("##### first ####");
    //Result 453
    first()?;

    println!("##### second ####");
    //Result 919
    second()?;
    Ok(())
}
