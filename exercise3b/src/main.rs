/*
The bit criteria depends on which type of rating value you want to find:

To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
For example, to determine the oxygen generator rating value using the same example diagnostic report from above:

Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
Then, to determine the CO2 scrubber rating value from the same example above:

Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.

Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine?

*/
use std::io::{BufReader, BufRead};
use std::fs::File;

fn count_ones_zeroes(f: File) -> ([i32; 12], [i32; 12]){
    let mut s = String::new();
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

    (zeroes, ones)
}

fn new_count(vec: &Vec<String>, index: usize) -> (i32, i32){
    let mut tup = (0, 0);
    for el in vec{
        match el.chars().nth(index).unwrap(){
            '1' => tup.0 += 1,
            '0' => tup.1 += 1,
            _ => continue,
        }
    }

    tup
}

fn main() {
    let counted = count_ones_zeroes(File::open("data.txt").unwrap());

    let f = File::open("data.txt").unwrap();
    let reader = BufReader::new(f);

    let vec: Vec<_> = reader.lines().collect();

    let mut ready_vec = vec![];
    for el in vec{
        ready_vec.push(el.unwrap());
    }

    let mut scrubber_vec = ready_vec.clone();
    let counted_zeroes = counted.0;
    let counted_ones = counted.1;

    let mut count = (counted_ones[0], counted_zeroes[0]);

    //Finding an oxygen rating
    for i in 0..counted_zeroes.len(){
        if count.0 > count.1{
            ready_vec.retain(|s| s.chars().nth(i).unwrap() == '1');
        }else if count.0 < count.1{
            ready_vec.retain(|s| s.chars().nth(i).unwrap() == '0');
        }else if count.0 == count.1 {
            ready_vec.retain(|s| s.chars().nth(i).unwrap() == '1');
        }
        if ready_vec.len() == 1{
            break;
        }
        if i == 11{
            count = new_count(&ready_vec, 11)
        }else{
            count = new_count(&ready_vec, i+1)
        }
    }

    count = (counted_ones[0], counted_zeroes[0]);

    //Finding a CO2 scrubber thing
    for i in 0..counted_zeroes.len(){
        if count.0 > count.1{
            scrubber_vec.retain(|s| s.chars().nth(i).unwrap() == '0');
        }else if count.0 < count.1{
            scrubber_vec.retain(|s| s.chars().nth(i).unwrap() == '1');
        }else if count.0 == count.1 {
            scrubber_vec.retain(|s| s.chars().nth(i).unwrap() == '0');
            println!("{:?} {}", count, i);
        }
        if scrubber_vec.len() == 1{
            break;
        }
        if i == 11{
            count = new_count(&scrubber_vec, 11);
        }else{
            count = new_count(&scrubber_vec, i+1);
            println!("{:?} {}", count, i);
        }
    }

    let tup = (i32::from_str_radix(ready_vec.get(0).unwrap().as_str(), 2).unwrap(), i32::from_str_radix(scrubber_vec.get(0).unwrap().as_str(), 2).unwrap());
    println!("Oxygen * CO2 = {}", tup.0*tup.1);
}