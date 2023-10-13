use std::fs::File;
use std::io::{BufRead, BufReader, Result};


pub fn read_lines_by_line(path: &str, mut f: impl FnMut(&String) -> Result<()>) -> Result<()> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let raw: String = line.expect("could  not read line");
        println!("read: {}", raw);
        f(&raw)?;
    }
    Ok(())
}