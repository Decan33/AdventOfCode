use std::io::{BufReader, BufRead};
use std::fs::File;
use std::char;

fn main() {
    let buffer = String::new();
    let mut s = String::new();
    let f = File::open("data.txt").unwrap();
    let reader = BufReader::new(f);

    let mut zeroes: [i32; 12] = [0; 12];
    let mut ones: [i32; 12] = [0; 12];

    for line in reader.lines() {
        s = line.unwrap();
        for i in 0..zeroes.len(){
            match s.chars().nth(i).unwrap(){
                '1' => ones[i] += 1,
                '0' => zeroes[i] += 1,
                _ => continue,
            }
        }
    }

    let mut gamma_rate = String::new();     //most common bits
    let mut epsilon_rate = String::new();   //least common bits

    for i in 0..zeroes.len(){
        if zeroes.get(i).unwrap() > ones.get(i).unwrap() {
            gamma_rate.push(char::from_digit(0, 10).unwrap());
            epsilon_rate.push(char::from_digit(1, 10).unwrap());
        }else {
            gamma_rate.push(char::from_digit(1, 10).unwrap());
            epsilon_rate.push(char::from_digit(0, 10).unwrap());
        }
    }

    println!("Gamma rate: {}, epsilon rate: {}", gamma_rate, epsilon_rate);
    let tup = (i32::from_str_radix(gamma_rate.as_str(), 2).unwrap(), i32::from_str_radix(epsilon_rate.as_str(), 2).unwrap());
    println!("Gamma rate(i32): {}, epsilon rate(i32): {}", tup.0, tup.1);
    println!("Gamma * epsilon = {}", tup.0*tup.1);


}
