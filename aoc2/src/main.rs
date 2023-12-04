use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc23/aoc2/src/file.txt").unwrap();
    let mut p1: i32 = 0;
    let mut p2: i32 = 0;

    for (index, val) in raw_file.lines().enumerate() {

        let a = val.split(";").collect::<Vec<&str>>();
        let mut is_valid: bool = true;
        let mut max_red: i32 = 0;
        let mut max_blue: i32 = 0;
        let mut max_green: i32 = 0;

        for i in a {

            let b = i.split(",").collect::<Vec<&str>>();

            for j in b {

                let c = j.split(" ").collect::<Vec<&str>>();
                let color = c.last().unwrap();
                let num: i32 = c.get(c.len()-2).unwrap().parse().unwrap();

                if ( color.starts_with("red") && num > 12 ) || 
                    ( color.starts_with("green") && num > 13 ) || 
                        ( color.starts_with("blue") && num > 14 ) {
                            is_valid = false;
                }

                if color.starts_with("blue") {
                    if max_blue < num {
                        max_blue = num;
                    }
                }

                if color.starts_with("red") {
                    if max_red < num {
                        max_red = num;
                    }
                }

                if color.starts_with("green") {
                    if max_green < num {
                        max_green = num;
                    }
                }

            }

        }

        p2 += max_blue * max_green * max_red;

        if is_valid {
            p1 += (index + 1) as i32;
        }

    }

    println!("p1 -> {p1}");
    println!("p2 -> {p2}");

}
