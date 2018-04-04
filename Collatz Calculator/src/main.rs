extern crate num;

use std::io;
use num::BigInt;


fn getinput() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput)
        .expect("Failed to read line!");
    userinput
}

fn collatz(mut n: BigInt, mut amount: BigInt) {

    while amount > BigInt::from(0) {
        let mut steps = 0;
        let mut greatest = n.clone();
        let original = n.clone();

        while n > BigInt::from(1) {
            steps += 1;
            if n.clone() % 2 == BigInt::from(0) {
                n = n.clone() / BigInt::from(2);
            } else {
                n = BigInt::from((3 * n) + 1);
            }

            if n > greatest {
                greatest = n.clone();
            }
        }

        n = original.clone() + BigInt::from(1);
        amount = amount - BigInt::from(1);
        println!("Original: {} - Steps: {} - Greatest: {}", original, steps, greatest);
    }
}

fn main() {
    println!("Enter the starting point:");
    let starting_point: BigInt = getinput().trim().parse::<BigInt>().unwrap();
    println!("---------------------------------------------");

    println!("Enter the end point:");
    let ending_point: BigInt = getinput().trim().parse::<BigInt>().unwrap();
    println!("---------------------------------------------");

    collatz(starting_point.clone(), ending_point - starting_point + BigInt::from(1));
}
