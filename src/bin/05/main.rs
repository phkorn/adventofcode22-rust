use adventofcode22::read_lines_by_line;
use itertools::Itertools;
use std::{io::Result, ops::Range};
use regex::{Regex};

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn print_vectors(vectors: &Vec<Vec<Option<char>>>){
    for (idx, v) in vectors.iter().enumerate() {
        print!("{}: ", idx+1);
        for e in v {
           print!("{} ", e.unwrap()) 
        }
        println!()
    }
}

fn add_to_vecs(vectors: &mut Vec<Vec<Option<char>>>, idx: usize, cargo: Option<char>){
    if vectors.get(idx).is_none() {
        vectors.insert(idx, Vec::new());
    }
    vectors[idx].push(cargo);
}

fn handle_move(vectors: &mut Vec<Vec<Option<char>>>, mov: Move) {
    println!("{:?}", mov);
    let slice: Vec<Option<char>>;
    {
        let v_from: &mut Vec<Option<char>> = vectors[mov.from-1].as_mut();
        let range = Range{ start: v_from.len() - mov.quantity, end: v_from.len()};
        let x = v_from.drain(range);
        slice = x.collect_vec();
    }
    let v_to: &mut Vec<Option<char>> = vectors[mov.to-1].as_mut();
    slice.iter().rev().for_each(|elem| {
        v_to.push(elem.clone());
    });
}

fn first() -> Result<()> {
    let path = "src/bin/05/input.txt";
    let regex_stack = Regex::new(r"(?m)\s?\[([A-Z])\]\s?|[^\d]\s(\s)\s").unwrap();
    let regex_moves = Regex::new(r"(?m)move (\d+) from (\d+) to (\d+)").unwrap();

    let mut vectors: Vec<Vec<Option<char>>> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();

    read_lines_by_line(path, |line| {
        match line {
         x if regex_stack.is_match(x) => {
            let iter = regex_stack.captures_iter(x);
            for (idx, mat) in iter.enumerate() {
                println!("{:?}", mat);
                match (mat.get(1), mat.get(2)) {
                    (Some(cargo), None) => add_to_vecs(&mut vectors, idx, cargo.as_str().chars().next()),
                    (None, Some(_)) => add_to_vecs(&mut vectors, idx, None),
                    _ => panic!("missing match clause in stack")
                }
            };
         },
         x if regex_moves.is_match(x) => {
            let iter = regex_moves.captures_iter(x);
            for mov in iter {
                moves.push(Move { quantity: mov.get(1).unwrap().as_str().parse::<usize>().unwrap(), from: mov.get(2).unwrap().as_str().parse::<usize>().unwrap(), to: mov.get(3).unwrap().as_str().parse::<usize>().unwrap() });     
            }
         },
         _ => println!("<skipped>")
        }
        Ok(())
    })?;
    for vec in vectors.iter_mut() {
        vec.reverse();
        vec.retain(|&x| x.is_some());
    }

    print_vectors(&vectors);
    for mov in moves {
        handle_move(&mut vectors, mov);
        print_vectors(&vectors);
        println!();
    }

    for v in vectors {
        print!("{}", v.last().unwrap().unwrap());
    }
    println!();
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
