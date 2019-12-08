use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::io::{BufRead, BufReader, Error};


fn main() {
    // https://adventofcode.com/2019/day/5
    let input = 1;
    let mut intcode = get_intcode().unwrap();
    println!("Intcode is {:?}", intcode);

    let output = parse_intcode(&mut intcode, input);
    println!("Output code is {:?}", output);

}

fn get_intcode () -> Result<Vec<i32>, Error> {
    let path = Path::new("resources/intcode.txt");
    let file = File::open(path)?;
    let mut buf_file = BufReader::new(file);
    let mut next_line = String::new();
    buf_file.read_line(&mut next_line)?;
    let intcode: Vec<i32> = next_line
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(intcode)
}

fn parse_intcode (intcode: &mut Vec<i32>, input: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();

    let mut frame: Vec<usize> = Vec::with_capacity(4);
    let mut skip;
    let mut opcode = get_opcode(intcode[0] as usize);

    match opcode {
        1 | 2  => skip = 4,
        3 | 4  => skip = 2,
        99 => skip = 0,
        _ => panic!("Cannot parse first opcode {:?}", opcode),
    }

    for i in 0..intcode.len() {
        let mut code = intcode[i] as usize;
        if skip > 0 {
            skip -= 1;
            frame.push(code);
        } else {
            //let mut instructions: Vec<i32> = Vec::with_capacity(4);
            let instructions = process_parameter_mode(&frame, intcode);

            match opcode {
                1 => intcode[frame[3]] = instructions[0] + instructions[1],
                2 => intcode[frame[3]] = instructions[0] * instructions[1],
                3 => intcode[frame[1]] = input,
                4 => output.push(instructions[0]),
                99 => break,
                _ => panic!("Cannot parse opcode {:?}", opcode),
            }

            frame.clear();
            // after running the instruction the next code may have changed
            code = intcode[i] as usize;
            opcode = get_opcode(code);
            frame.push(code);

            match opcode {
                1 | 2 => skip = 3,
                3 | 4 => skip = 1,
                99 => skip = 0,
                _ => {
                    println!("Cannot parse opcode {:?}", opcode);
                    break;
                }
            }
        }
    }
    output
}

fn process_parameter_mode(frame: &Vec<usize>, memory: &Vec<i32>) -> Vec<i32> {
    // zero pad int
    let int_string = format!("{:05}", frame[0]);
    let mut instructions = Vec::with_capacity(2);

    let sequence: Vec<i32> = int_string
        .split_terminator("")
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut index = 2;

    for fram in frame[1..].iter() {
        if index == 0 {
            break;
        }
        if sequence[index] == 0 {
            instructions.push(memory[*fram as usize]);
        } else if sequence[index] == 1 {
            instructions.push(*fram as i32);
        } else {
            panic!{"Cannot process mode {:?}", sequence[index]}
        }
        index -= 1;
    }

    instructions
}

fn get_opcode(instruction: usize) -> usize {
    // zero pad int
    let int_string = format!("{:05}", instruction);
    int_string[4..5].parse::<usize>().unwrap()
}
