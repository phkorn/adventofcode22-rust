use adventofcode22::read_lines_by_line;
use std::io::Result;

fn first() -> Result<()> {
    let path = "src/bin/03/input.txt";
    let mut sum = 0;

    read_lines_by_line(path, |line| {
        let compartment_1 = &line[0..line.len() / 2];
        let compartment_2 = &line[line.len() / 2..line.len()];
        print!("first {}, second {}, ", compartment_1, compartment_2);

        for c in compartment_1.chars() {
            match c {
                c if compartment_2.contains(c) && c.is_ascii_lowercase() => {
                    let priority = c as u32 - 96;
                    sum += priority;
                    println!("{} is in both, priority {}", c, priority);
                    break;
                }
                c if compartment_2.contains(c) && c.is_ascii_uppercase() => {
                    let priority = c as u32 - 38;
                    sum += priority;
                    println!("{} is in both, priority {}", c, priority);
                    break;
                }
                _ => (),
            }
        }
        Ok(())
    })?;
    println!("sum: {}", sum);
    Ok(())
}

fn second() -> Result<()> {
    let path = "src/bin/03/input.txt";
    let mut sum = 0;
    let mut vec: Vec<String> = Vec::new();

    read_lines_by_line(path, |line| {
        if vec.len() == 2 {
            vec.push(line.clone());
            for c in vec[0].chars() {
                match c {
                    c if vec[1].contains(c) && vec[2].contains(c) && c.is_ascii_lowercase() => {
                        let priority = c as u32 - 96;
                        sum += priority;
                        println!("{} is in all three, priority {}", c, priority);
                        break;
                    }
                    c if vec[1].contains(c) && vec[2].contains(c) && c.is_ascii_uppercase() => {
                        let priority = c as u32 - 38;
                        sum += priority;
                        println!("{} is in all three, priority {}", c, priority);
                        break;
                    }
                    _ => (),
                }
            }
            vec.clear();
        } else {
            vec.push(line.clone());
        }
        Ok(())
    })?;
    println!("sum: {}", sum);
    Ok(())
}

fn main() -> Result<()> {
    println!("##### first ####");
    //7727
    first()?;

    println!("##### second ####");
    //2609
    second()?;
    Ok(())
}
