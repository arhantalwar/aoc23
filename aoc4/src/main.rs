use std::fs;

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc23/aoc4/src/file.txt").unwrap();
    let mut p1: i32 = 0;
    let mut occurance: Vec<i32> = Vec::new();
    let mut count: Vec<i32> = Vec::new();

    for l in raw_file.lines() {

        let a = l.split("|").collect::<Vec<&str>>();
        let b = a.get(0).unwrap().split(" ").collect::<Vec<&str>>();
        let c = a.get(1).unwrap().split(" ").collect::<Vec<&str>>();
        let mut subp: i32 = 0;
        let mut subp2: i32 = 0;

        let mut d: Vec<i32> = Vec::new();
        let mut e: Vec<i32> = Vec::new();

        for i in b {

            let num = i.parse::<i32>();

            if num.is_ok() {
                d.push(num.clone().unwrap());
            }

        }

        for i in c {

            let num = i.parse::<i32>();

            if num.is_ok() {
                e.push(num.clone().unwrap());
            }

        }

        for i in d {

            if e.contains(&i) {
                if subp == 0 {
                    subp += 1;
                } else {
                    subp *= 2;
                }
                subp2 += 1;
            }

        }

        p1 += subp;

        occurance.push(1);
        count.push(subp2);

    }


    println!("P1 -> {:?}", p1);

    for ind in 0..occurance.len() {

        for _ in 0..occurance[ind] {

            for j in (ind+1)..((ind+1) + count[ind] as usize) {

                let a = occurance.get_mut(j).unwrap();
                *a += 1;

            }

        }

    }


    let sum: i32 = occurance.iter().sum();

    println!("P2 -> {:?}", sum);


}
