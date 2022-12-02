use std::cmp::max;
use std::env;
use std::fs;


fn main() {
    let data = fs::read_to_string("./data/input4.txt").expect("Unable to read file");
    //println!("{}", data);


    let mut sum: u32 = 0;
    for line in data.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        println!("{}", tokens[1]);
        /**
        match tokens[1] {
            "X" => {
                if tokens[0] == "A"{
                    sum +=3;
                } else if tokens[0] == "B"{
                    sum += 0;
                } else {
                    sum += 6;
                }
                sum += 1
            },
            "Y" => {
                if tokens[0] == "A"{
                    sum +=6;
                } else if tokens[0] == "B"{
                    sum += 3;
                } else {
                    sum += 0;
                }
                sum += 2
            },
            "Z" => {
                if tokens[0] == "A"{
                    sum +=0;
                } else if tokens[0] == "B"{
                    sum += 6;
                } else {
                    sum += 3;
                }
                sum += 3
            },
            &_ => println!("nothing")
        }
        **/
        match tokens[0] {
            "A" => {
                if tokens[1] == "X"{
                    sum +=0;
                    sum +=3;
                } else if tokens[1] == "Y"{
                    sum += 3;
                    sum += 1;
                } else {
                    sum += 6;
                    sum += 2;
                }
            },
            "B" => {
                if tokens[1] == "X"{
                    sum +=0;
                    sum += 1;
                } else if tokens[1] == "Y"{
                    sum += 3;
                    sum += 2;
                } else {
                    sum += 6;
                    sum += 3;
                }
            },
            "C" => {
                if tokens[1] == "X"{
                    sum += 0;
                    sum += 2;
                } else if tokens[1] == "Y"{
                    sum += 3;
                    sum += 3
                } else {
                    sum += 6;
                    sum += 1;
                }
            },
            &_ => println!("nothing")
        }

    }
    println!("{}",sum)
}
