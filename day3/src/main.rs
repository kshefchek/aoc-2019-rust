use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let (first_wire, second_wire) = get_directions().unwrap();

    let first_coords = directions_to_coords(&first_wire);
    let second_coords = directions_to_coords(&second_wire);

    let first_wire: HashMap<_, _> = first_coords
        .into_iter()
        .map(|x| ((x.0, x.1), x.2))
        .collect();

    let second_wire: HashMap<_, _> = second_coords
        .into_iter()
        .map(|x| ((x.0, x.1), x.2))
        .collect();

    let first_distances: HashSet<&(i32, i32)> = HashSet::from_iter(first_wire.keys());
    let second_distances: HashSet<&(i32, i32)> = HashSet::from_iter(second_wire.keys());

    let intersection: HashSet<_> = first_distances.intersection(&second_distances).cloned().collect();

    let min_dist = intersection.iter().map(|x| x.0.abs() + x.1.abs()).min();

    println!("Part one: {:?}", min_dist.unwrap());

    let min_sig_delay = intersection
        .iter()
        .map(|x| first_wire.get(&(x.0,x.1)).unwrap()
            + second_wire.get(&(x.0,x.1)).unwrap())
        .min();

    println!("Part two: {:?}", min_sig_delay.unwrap());

}

fn get_directions() -> Result<(Vec<String>,Vec<String>), Error> {
    let path = Path::new("resources/directions.txt");
    let file = File::open(path)?;
    let mut buf_file = BufReader::new(file);
    let mut next_line = String::new();
    buf_file.read_line(&mut next_line)?;
    let first_wire = next_line
        .trim()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    next_line.clear();

    buf_file.read_line(&mut next_line)?;
    let second_wire = next_line
        .trim()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    Ok((first_wire, second_wire))
}

fn directions_to_coords(directions: &Vec<String>) -> Vec<(i32, i32, i32)> {
    let mut x= 0;
    let mut y= 0;
    let mut total_dist = 0;
    let mut coordinates = vec![];
    for direction in directions {
        let chars: Vec<char> = direction.chars().collect();
        let orientation = chars[0];
        let distance: u32 = chars[1..].into_iter().collect::<String>().parse().unwrap();

        for _ in 0..distance {
            total_dist += 1;
            if orientation == 'R' {
                x = x + 1;
            } else if orientation == 'L' {
                x = x - 1;
            } else if orientation == 'U' {
                y = y + 1;
            } else if orientation == 'D' {
                y = y - 1;
            };
            coordinates.push((x, y, total_dist));
        }
    }

    coordinates
}