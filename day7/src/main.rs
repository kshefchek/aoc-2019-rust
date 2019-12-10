use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::io::{BufRead, BufReader, Error};


fn main() {
    // https://adventofcode.com/2019/day/7
    let intcode = get_intcode().unwrap();

    let phases = vec![0,1,2,3,4];
    let input = 0;

    let mut phase_setting = Vec::with_capacity(5);
    let mut accum = Vec::new();
    let mut level = 0;

    get_signals(&intcode, &phases, &input, &mut phase_setting, &mut level, &mut accum);

    let max: i32 = accum.iter().map(|x| x.1).max().unwrap();

    println!("{:?}", max);
}

fn get_signals (
    intcode: &Vec<i32>,
    phases: &Vec<i32>,
    input: &i32,
    phase_setting: &mut Vec<i32>,
    level: &mut u8,
    accum: &mut Vec<(Vec<i32>, i32)>
) {

    let intcopy = intcode.clone();

    for phase in phases {
        phase_setting.push(*phase);

        let mut inputs = vec![*phase, *input].into_iter();
        let (new_intcode, new_input) = parse_intcode(&intcopy, &mut inputs);
        let new_phases: Vec<i32> = phases
            .iter()
            .cloned()
            .filter(|x| x != phase)
            .collect();

        if new_phases.len() == 0 {
            let final_phases = phase_setting.to_vec();
            accum.push((final_phases, new_input));
        } else {
            *level += 1;
            get_signals(&new_intcode, &new_phases, &new_input, phase_setting, level, accum);
        }
    }
    if *level > 0 {
        *level -= 1;
        phase_setting.truncate(*level as usize);
    }
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

fn parse_intcode <I>(intcode: &Vec<i32>, input: &mut I) -> (Vec<i32>, i32)
    where I: Iterator<Item = i32>
{
    let mut intcopy = intcode.clone();
    let mut output: Vec<i32> = Vec::new();
    let mut frame: Vec<usize> = Vec::with_capacity(4);
    let mut buff_iter = 0..intcode.len();

    while let Some(i) = buff_iter.next() {
        frame.clear();
        let opcode = get_opcode(intcopy[i] as usize);
        match opcode {
            1 | 2 | 7 | 8 => {
                frame = intcopy[i..i+4].into_iter().map(|val| *val as usize).collect();
                buff_iter.nth(2);
            },
            3 | 4 => {
                frame = intcopy[i..i+2].into_iter().map(|val| *val as usize).collect();
                buff_iter.nth(0);
            },
            5 | 6 => {
                frame = intcopy[i..i+3].into_iter().map(|val| *val as usize).collect();
            },
            99 => break,
            _ =>  {
                println!("Cannot parse opcode {:?}", opcode);
                break;
            },
        }
        let instructions = process_parameter_mode(&frame, &intcopy);

        match opcode {
            1 => intcopy[frame[3]] = instructions[0] + instructions[1],
            2 => intcopy[frame[3]] = instructions[0] * instructions[1],
            3 => intcopy[frame[1]] = input.next().unwrap(),
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
                    intcopy[frame[3]] = 1;
                } else {
                    intcopy[frame[3]] = 0;
                }
            },
            8 => {
                if instructions[0] == instructions[1] {
                    intcopy[frame[3]] = 1;
                } else {
                    intcopy[frame[3]] = 0;
                }
            },
            99 => break,
            _ => panic!("Cannot parse opcode {:?}", opcode),
        }
        if output.len() > 0 {
            break;
        }
    }
    (intcopy, output[0])
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
