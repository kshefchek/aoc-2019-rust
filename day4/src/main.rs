fn main() {
    println!("Part one: {}", part_one());

    println!("Part two: {}", part_two());
}

fn part_one() -> i32 {
    let mut count = 0;
    for pwd in 171309..=643603 {
        let mut score = 0;
        let sequence: Vec<i32> = pwd
            .to_string()
            .split_terminator("")
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        for (index, val) in sequence[1..].iter().enumerate() {
            if val < &sequence[index] {
                score = 0;
                break;
            }
            if val == &sequence[index] {
                score += 1;
            }
        }
        if score >= 1 {
            count += 1;
        }
    }
    count
}

fn part_two() -> i32 {
    let mut count = 0;
    for pwd in 171309..=643603 {
        let mut is_valid = true;

        // zero pad int
        let int_string = format!("{:07}", pwd);

        let mut sequence: Vec<i32> = int_string
            .split_terminator("")
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        for (index, val) in sequence[1..].iter().enumerate() {
            if val < &sequence[index] {
                is_valid = false;
                break
            }
        }
        if !is_valid {continue;}
        is_valid = false;

        // pad to the right
        sequence.push(0);

        for (index, val) in sequence[3..].iter().enumerate() {
            if &sequence[index+1]  == &sequence[index+2]
                  && &sequence[index+1] != &sequence[index]
                  && &sequence[index+1] != val {
                is_valid = true;
            }
        }
        if is_valid {
            count += 1;
        }
    }
    count
}