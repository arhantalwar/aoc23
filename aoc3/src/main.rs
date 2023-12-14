use std::{fs, collections::HashMap};

fn main() {

    let raw_file = fs::read_to_string("/home/arhant/aoc23/aoc3/src/file.txt").unwrap();
    let mut grid: Vec<((usize, usize), i32)> = Vec::new();
    let mut hashy: HashMap<(isize, isize), (i32, i32, i32)> = HashMap::new();
    let mut map: Vec<Vec<&str>> = Vec::new();
    let symbols: Vec<&str> = vec!["#", "@", "/", "*", "+", "-", "=", "$", "%", "&"];
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;

    for (i_index, i) in raw_file.lines().enumerate() {

        let mut num: String = String::new();
        let mut a = i.split("").collect::<Vec<&str>>();

        a.iter().enumerate().for_each(|(x_index, x)| {
            if x.parse::<i32>().is_ok() {
                num.push_str(x);
            } else if num != "" {
                grid.push(( (i_index, x_index - num.len() - 1), num.parse::<i32>().unwrap() ));
                num.clear();
            }
        });

        a.remove(0);
        a.pop();

        map.push(a.clone());

    }

    for m in &grid {

        let ((y, x), val) = m;
        let mut x_vec: Vec<isize> = Vec::new();
        let y_vec: Vec<isize> = vec![-1, 0, 1];

        // println!("{:?}", m);

        x_vec.push(*x as isize - 1);

        for i in *x..=(x + val.to_string().len()) {
            x_vec.push(i as isize);
        }

        for row in &x_vec {
            for col in &y_vec {
                if let Some(_) = map.get((*col + (*y as isize)) as usize) {
                    if let Some(j) = map.get((*col + (*y as isize)) as usize).unwrap().get(*row as usize) {
                        if symbols.contains(j) {
                            // println!("{:?} {:?} {:?}", j, (*col + (*y as isize)), (*row));
                            hashy.entry((*col + (*y as isize), *row)).and_modify(|(e1, e2, e3)| {
                                if *e1 == 1 {
                                    *e3 = *val;
                                }
                                *e1 += 1;
                            }).or_insert((1, *val, 0));
                            p1 += *val as u64;
                        }
                    }
                }
            }
        }

    }

    for i in hashy {

        let ((_, _), (a, b, c)) = i;

        if a == 2 {
            p2 += (b * c) as u64;
        }

    }

    println!("P1 -> {:?}", p1);
    println!("P2 -> {:?}", p2);

}
