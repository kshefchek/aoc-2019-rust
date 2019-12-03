use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::io::{BufRead, BufReader, Error};


fn main() {
    // https://adventofcode.com/2019/day/2

    let intcode = get_intcode().unwrap();
    let instructions = parse_intcode(&intcode, 12, 2);
    println!("Code at position 0 is {}", instructions[0]);

    for noun in  0..99 {
        for verb in 0..99 {
            let output = parse_intcode(&intcode, noun, verb)[0];
            if output == 19690720 {
                println!("Noun is {}", noun);
                println!("Verb is {}", verb);
                break;
            }
        }
    }
}

fn get_intcode () -> Result<Vec<u32>, Error> {
    let path = Path::new("resources/intcode.txt");
    let file = File::open(path)?;
    let mut buf_file = BufReader::new(file);
    let mut next_line = String::new();
    buf_file.read_line(&mut next_line)?;
    let intcode: Vec<u32> = next_line
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(intcode)
}

fn parse_intcode (intcode: &Vec<u32>, noun: u32, verb: u32) -> Vec<u32> {

    let mut instructions = intcode.clone();
    instructions[1] = noun;
    instructions[2] = verb;

    let mut frame: Vec<usize> = Vec::with_capacity(4);
    let mut index: u32 = 0;

    for code in intcode {
        if index % 4 == 0 && frame.len() != 0 {
            match frame[0] {
                99 => break,
                1 => instructions[frame[3]] = instructions[frame[1]] + instructions[frame[2]],
                2 => instructions[frame[3]] = instructions[frame[1]] * instructions[frame[2]],
                _ => panic!("Can't handle opcode"),
            }
            frame.clear();
            frame.push(*code as usize);
        } else {
            frame.push(*code as usize);
        }
        index += 1;
    }

    instructions
}
