use adventofcode22::read_lines_by_line;
use std::io::Result;

fn first() -> Result<()> {
    let path = "src/bin/XX/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line| {
        Ok(())
    })?;
    Ok(())
}

fn second() -> Result<()> {
    let path = "src/bin/XX/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line| {
        Ok(())
    })?;
    Ok(())
}

fn main() -> Result<()> {
    println!("##### first ####");
    //Result
    first()?;

    println!("##### second ####");
    //Result
    second()?;
    Ok(())
}
