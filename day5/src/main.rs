use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::io::{BufRead, BufReader, Error};


fn main() {
    // https://adventofcode.com/2019/day/5
    let mut intcode = get_intcode().unwrap();
    println!("Intcode is {:?}", intcode);

    let mut input = 1;
    let mut output = parse_intcode(&mut intcode, input);
    println!("Output code for part one {:?}", output); // 13818007

    input = 5;
    // Reset the intcode instructions
    intcode = get_intcode().unwrap();
    output = parse_intcode(&mut intcode, input);
    println!("Output code for part two {:?}", output); // 3176266

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
    let mut buff_iter = 0..intcode.len();

    while let Some(i) = buff_iter.next() {
        frame.clear();
        let opcode = get_opcode(intcode[i] as usize);
        match opcode {
            1 | 2 | 7 | 8 => {
                frame = intcode[i..i+4].into_iter().map(|val| *val as usize).collect();
                buff_iter.nth(2);
            },
            3 | 4 => {
                frame = intcode[i..i+2].into_iter().map(|val| *val as usize).collect();
                buff_iter.nth(0);
            },
            5 | 6 => {
                frame = intcode[i..i+3].into_iter().map(|val| *val as usize).collect();
            },
            99 => break,
            _ =>  {
                println!("Cannot parse opcode {:?}", opcode);
                break;
            },
        }
        let instructions = process_parameter_mode(&frame, intcode);

        match opcode {
            1 => intcode[frame[3]] = instructions[0] + instructions[1],
            2 => intcode[frame[3]] = instructions[0] * instructions[1],
            3 => intcode[frame[1]] = input,
            4 => output.push(instructions[0]),
            5 => {
                if instructions[0] != 0 {
                    let skip_n = instructions[1] - i as i32 - 2;
                    buff_iter.nth(skip_n as usize);
                } else {
                    buff_iter.nth(1);
                }
            },
            6 => {
                if instructions[0] == 0 {
                    let skip_n = instructions[1] - i as i32 - 2;
                    buff_iter.nth(skip_n as usize);
                } else {
                    buff_iter.nth(1);
                }
            },
            7 => {
                if instructions[0] < instructions[1] {
                    intcode[frame[3]] = 1;
                } else {
                    intcode[frame[3]] = 0;
                }
            },
            8 => {
                if instructions[0] == instructions[1] {
                    intcode[frame[3]] = 1;
                } else {
                    intcode[frame[3]] = 0;
                }
            },
            99 => break,
            _ => panic!("Cannot parse opcode {:?}", opcode),
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
            instructions.push(memory[*fram]);
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
