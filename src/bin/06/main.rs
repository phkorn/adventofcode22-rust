use adventofcode22::read_lines_by_line;
use std::io::Result;

fn lookup(str: &str, idx: usize, substring_length: usize) -> Option<usize> {
    if idx == str.len() - 1 {
        return None;
    }
    let substr = &str[idx..idx + substring_length];
    let any_occurs_multiple_times = substr
        .chars()
        .any(|c: char| substr.match_indices(c).count() > 1);
    if !any_occurs_multiple_times {
        return Some(idx + substring_length);
    }
    lookup(str, idx + 1, substring_length)
}

fn get_start_of_datastream(substring_length: usize) -> Result<()> {
    let path = "src/bin/06/input.txt";

    read_lines_by_line(path, |line| {
        let res = lookup(line, 0, substring_length);
        println!("result: {:?}", res);
        Ok(())
    })?;
    Ok(())
}

fn intro() -> Result<()> {
    let examples: Vec<(&str, usize)> = vec![
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];
    examples
        .iter()
        .for_each(|x| println!("{} result:{:?} expected:{}", x.0, lookup(x.0, 0, 4), x.1));
    Ok(())
}

fn first() -> Result<()> {
    get_start_of_datastream(4)?;
    Ok(())
}

fn second() -> Result<()> {
    get_start_of_datastream(14)?;
    Ok(())
}

fn main() -> Result<()> {
    println!("##### intro ####");
    intro()?;

    println!("##### first ####");
    //Result 1175
    first()?;

    println!("##### second ####");
    //Result 3217
    second()?;
    Ok(())
}
