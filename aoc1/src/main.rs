use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc23/aoc1/src/file.txt").unwrap();
    let mut cal_val: i32 = 0;

    for line in raw_file.lines() {

        let mut p2: Vec<i32> = Vec::new();
        let word = String::from(line);

        for i in 0..word.len() {

            let slc = &word[i..];
            let ch = &word[i..i+1].parse::<i32>();

            if ch.is_ok() {
                p2.push(ch.clone().unwrap());
            }

            for (index, w) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].iter().enumerate() {

                if slc.starts_with(w) {
                    p2.push((index+1) as i32);
                }
                
            }

        }

        cal_val += p2.get(0).unwrap() * 10 + p2.get(p2.len() - 1).unwrap();

    }

    println!("P2 -> {:?}", cal_val);

}
