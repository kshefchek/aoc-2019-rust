use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::iter::IntoIterator;
use std::io::{BufRead, BufReader, Error};


fn main() {
    // https://adventofcode.com/2019/day/1
    println!("Fuel requirement: {:?}", part_one()
        .expect("Could not calculate fuel requirement"));

    println!("Total Fuel requirement: {:?}", part_two()
        .expect("Could not calculate fuel requirement"));
}

fn part_one () -> Result<i32, Error> {
    // Create a path to the desired file
    let path = Path::new("resources/module-mass.txt");
    let file = File::open(path)?;
    let buf_file = BufReader::new(file);

    get_fuel_requirement(buf_file.lines())
}

fn part_two () -> Result<i32, Error> {
    // Create a path to the desired file
    let path = Path::new("resources/module-mass.txt");
    let file = File::open(path)?;
    let buf_file = BufReader::new(file);

    get_total_fuel_requirement(buf_file.lines())
}

fn get_total_fuel_requirement <I, T>(iter: I) -> Result<i32, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: AsRef<str>,
{
    let mut total_fuel = 0;

    for mass in iter {
        let mut fuel_req = get_fuel_requirement(vec![mass])?;
        let mut next_req = fuel_req;
        while next_req > 0 {
            next_req = get_fuel_requirement(vec![Ok(next_req.to_string())])?;
            if next_req > 0 {
                fuel_req += next_req;
            }
        }
        total_fuel += fuel_req;
    }

    Ok(total_fuel)
}

fn get_fuel_requirement <I, T>(iter: I) -> Result<i32, Error>
    where
        I: IntoIterator<Item = Result<T, Error>>,
        T: AsRef<str>,
{
    let fuel_req: f64 = iter
        .into_iter()
        .fold(0.0, |accum, item| accum +
            (item.unwrap().as_ref().parse::<f64>().unwrap() / 3.0).floor() - 2.0
        );
    Ok(fuel_req as i32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let case_1 = ("12",     2);
        let case_2 = ("14",     2);
        let case_3 = ("1969",   654);
        let case_4 = ("100756", 33583);

        assert_eq!(get_fuel_requirement(vec![Ok(case_1.0)]).unwrap(), case_1.1);
        assert_eq!(get_fuel_requirement(vec![Ok(case_2.0)]).unwrap(), case_2.1);
        assert_eq!(get_fuel_requirement(vec![Ok(case_3.0)]).unwrap(), case_3.1);
        assert_eq!(get_fuel_requirement(vec![Ok(case_4.0)]).unwrap(), case_4.1);
    }

    #[test]
    fn test_part_two() {
        let case_1 = ("14",     2);
        let case_2 = ("1969",   966);
        let case_3 = ("100756", 50346);
        let case_4 = (vec![Ok("100756"), Ok("1969")], 51312);

        assert_eq!(get_total_fuel_requirement(vec![Ok(case_1.0)]).unwrap(), case_1.1);
        assert_eq!(get_total_fuel_requirement(vec![Ok(case_2.0)]).unwrap(), case_2.1);
        assert_eq!(get_total_fuel_requirement(vec![Ok(case_3.0)]).unwrap(), case_3.1);
        assert_eq!(get_total_fuel_requirement(case_4.0).unwrap(), case_4.1);
    }
}