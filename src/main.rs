use std::cmp::max;
use std::env;
use std::fs;


fn main() {
    let data = fs::read_to_string("./data/input2.txt").expect("Unable to read file");
    //println!("{}", data);

    let mut maxSum1: u32 = 0;
    let mut maxSum2: u32 = 0;
    let mut maxSum3: u32 = 0;
    let mut intermediateSum: u32 = 0;
    for line in data.lines() {
        let tokens: String = line.split_whitespace().collect();

        if !(tokens == "") {
            intermediateSum += tokens.parse::<u32>().unwrap();

        } else {
            if intermediateSum > maxSum1 {
                maxSum3 = maxSum2;
                maxSum2 = maxSum1;
                maxSum1 = intermediateSum;
            } else if intermediateSum > maxSum2 {
                maxSum3 = maxSum2;
                maxSum2 = intermediateSum;
            } else if intermediateSum > maxSum3 {
                maxSum3 = intermediateSum;
            }

            intermediateSum = 0;
        }

    }
    println!("{}", maxSum1+maxSum2+maxSum3)
}
