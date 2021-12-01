use std::env;
use std::fs;

fn main() {
    let argv: Vec<String> = env::args().collect();
    assert_eq!(argv.len(), 2, "Not enough (or too many) arguments!");

    let input = fs::read_to_string(argv[1].clone()).expect("Error reading file");

    let strvec: Vec<&str> = input.split("\n").collect();
    let mut intvec: Vec<u32> = [].to_vec();
    {
        let mut i = 0;
        loop {
            if i >= strvec.len() {
                break;
            }
            intvec.push(strvec[i].parse().unwrap());
            i += 1;
        }
    }
    part1(intvec.clone());
    part2(intvec);
}

fn part2(intvec: Vec<u32>) {
    let mut combinedvec: Vec<u32> = [].to_vec();
    {
        let mut i = 2;
        loop {
            if i >= intvec.len() {
                break;
            }
            combinedvec.push(intvec[i] + intvec[i - 1] + intvec[i - 2]);
            i += 1;
        }
    }
    let mut bigger = 0;
    {
        let mut i = 1;
        loop {
            if i >= combinedvec.len() {
                break;
            }
            if combinedvec[i] > combinedvec[i - 1] {
                bigger += 1;
            }
            i += 1;
        }
    }
    println!("Part 2: {}", bigger);
}

fn part1(intvec: Vec<u32>) {
    let mut bigger = 0;
    {
        let mut i = 1;
        loop {
            if i >= intvec.len() {
                break;
            }
            if intvec[i] > intvec[i - 1] {
                bigger += 1;
            }
            i += 1;
        }
    }
    println!("Part 1: {}", bigger);
}
