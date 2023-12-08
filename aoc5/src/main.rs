use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc23/aoc5/src/file.txt").unwrap();
    let mut big_vec: Vec<Vec<&str>> = Vec::new();
    let mut sml_vec: Vec<&str> = Vec::new();

    let seeds: Vec<i64> = vec![2041142901, 113138307, 302673608, 467797997, 1787644422, 208119536, 
                               143576771, 99841043, 4088720102, 111819874, 946418697, 13450451, 
                               3459931852, 262303791, 2913410855, 533641609, 2178733435, 26814354, 1058342395, 175406592];

    let mut location: Vec<i64> = Vec::new();

    for i in raw_file.lines() {

        if i == "" {
            big_vec.push(sml_vec.clone());
            sml_vec.clear();
        } else {
            sml_vec.push(i);
        }

    }

    let chunk = seeds.chunks(2);

    for k in chunk {

        let a = k.get(0).unwrap();
        let b = k.get(1).unwrap();

        for z in *a..(a+b) {

            let mut to_find: i64 = z;

            for i in &big_vec {

                for j in i {

                    let a: Vec<&str> = j.split(" ").collect();
                    let a0: i64 = a.get(0).unwrap().parse().unwrap();
                    let a1: i64 = a.get(1).unwrap().parse().unwrap();
                    let a2: i64 = a.get(2).unwrap().parse().unwrap();

                    if (to_find >= a1) && (to_find < (a1 + a2)) {

                        let diff: i64 = a0-a1;
                        to_find += diff;
                        break;

                    }

                }

            }

            location.push(to_find);

        }

    }

    location.sort();

    let sml: &i64 = location.get(0).unwrap();
    println!("P1 -> {:?}", sml);

}
