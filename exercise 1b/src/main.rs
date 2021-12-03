use std::io::{ BufReader, BufRead};
use std::fs::File;

fn main() {
    let f = File::open("data.txt").unwrap();
    let reader = BufReader::new(f);

    let vector: Vec<i32> = reader
    .lines()
    .map(|x| x.unwrap().trim().parse().unwrap()).collect();

    let mut accum_prev = vector.get(0).unwrap() + vector.get(1).unwrap() + vector.get(2).unwrap();
    let mut accum_next = 0;
    let mut it = 0;

    for i in 1..vector.len() {
        for j in 0..3 {
            if vector.get(i+j) == None {
                break;
            }
            accum_next += vector.get(i+j).unwrap();
        }

        if accum_next - accum_prev > 0 {
            it += 1;
        }

        accum_prev = accum_next;
        accum_next = 0;
    }
    
    println!("{}", it);
}