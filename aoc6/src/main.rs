use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc23/aoc6/src/file.txt").unwrap();

    let mut t_vec: Vec<u64> = Vec::new();
    let mut d_vec: Vec<u64> = Vec::new();
    let mut p1: u64 = 1;
    let mut p2: u64 = 1;

    for (i_index, i) in raw_file.lines().enumerate() {

        i.split(" ").for_each(|x| {
            if let Ok(i) = x.parse::<u64>() {
                if i_index == 0 {
                    t_vec.push(i);
                } else {
                    d_vec.push(i);
                }
            }
        });

    }

    for (index, val) in t_vec.iter().enumerate() {

        let mut how_many: u64 = 0;

        for i in 1..*val {

            if (val-i) * i > *d_vec.get(index).unwrap() {
                how_many += 1;
            }

        }

        p1 *= how_many;

    }

    println!("P1 -> {:?}", p1);

    let t_vec_str: String = t_vec.iter().map(|x| x.to_string()).collect();
    let d_vec_str: String = d_vec.iter().map(|x| x.to_string()).collect();

    t_vec.clear();
    d_vec.clear();

    t_vec.push(t_vec_str.parse::<u64>().unwrap());
    d_vec.push(d_vec_str.parse::<u64>().unwrap());

    for (index, val) in t_vec.iter().enumerate() {

        let mut how_many: u64 = 0;

        for i in 1..*val {

            if (val-i) * i > *d_vec.get(index).unwrap() {
                how_many += 1;
            }

        }

        p2 *= how_many;

    }

    println!("P2 -> {:?}", p2);


}
