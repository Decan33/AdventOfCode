/*--- Day 2: Dive! ---
Now, you need to figure out how to pilot this thing.

It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:

forward X increases the horizontal position by X units.
down X increases the depth by X units.
up X decreases the depth by X units.
Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.

The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:

forward 5
down 5
forward 8
up 3
down 8
forward 2
Your horizontal position and depth both start at 0. The steps above would then modify them as follows:

forward 5 adds 5 to your horizontal position, a total of 5.
down 5 adds 5 to your depth, resulting in a value of 5.
forward 8 adds 8 to your horizontal position, a total of 13.
up 3 decreases your depth by 3, resulting in a value of 2.
down 8 adds 8 to your depth, resulting in a value of 10.
forward 2 adds 2 to your horizontal position, a total of 15.
After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)

Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
*/

use std::io::{ BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Position(i32, i32);  //(horizontal, depth)

fn main() {
    let mut buffer = String::new();
    //let mut copy_buffer = String::new();
    let f = File::open("data.txt").unwrap();
    let mut pos = Position(0, 0);

    let mut reader = BufReader::new(f);

    loop{
        match reader.read_line(&mut buffer){
            Ok(bytes_read) => {
                if bytes_read == 0{
                    break;
                }
                
                let mut it = buffer.split(' ');
                //println!("how_much: {:?}", it);
                

                match it.next(){
                    Some(command) => {
                        let how_much = it.next().unwrap();
                        println!("how_much: {}", how_much);
                        let parsed_amount: i32 = how_much.trim().parse().unwrap();
                        if command.to_owned() == "forward" {
                            pos.0 += parsed_amount;
                        }else if command.to_owned() == "down" {
                            pos.1 += parsed_amount;
                        }else if command.to_owned() == "up" {
                            pos.1 -= parsed_amount;
                        }
                    },

                    None => continue
                };
            },
            Err(_) => break
        }
        buffer.clear();
    }

    println!("{:?}", pos);
    println!("Result is: {:?}", pos.0 * pos.1);

}
