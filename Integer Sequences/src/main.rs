//Sequence Generator - Generates the given number for a sequence at specified index
/*
Commands:
    fib [N1, N2, N3, ...] - Returns the Fibonacci number(s) at the specified indices.
    luc [N1, N2, N3, ...] - Returns the Lucas number(s) at the specified indices.
    brady [N1, N2, N3, ...] - Returns the Brady number(s) at the specified indices (From Numberphile).
    reca [N1, N2, N3, ...] - Returns the corresponding Recaman numbers.
    
    exit - Exits the program.
*/
extern crate num;
extern crate termion;

use std::io;
use std::io::Read;
use num::BigInt;
use termion::clear;

//Get userinput
fn getinput() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput)
        .expect("Failed to read line!");
    userinput
}

//Sequence functions
//FN: add_prev_two - For sequences that are formed by adding the two previous terms
fn add_prev_two(seq_type: &str, index_string: Vec<String>) -> Vec<BigInt> {
    //Parse vector and get max
    let mut i: Vec<usize> = Vec::new();
    for index in index_string {
        match index.trim().parse::<usize>() {
            Ok(x) => i.push(x),
            Err(_) => break
        }
    }

    //Set the max_index if possible, otherwise it remains 0 if the user did not input a valid number
    let mut max_index: usize = 0;
    if i.len() != 0 {
        max_index = *i.iter().max().unwrap();
    }

    //Form the first few terms
    let mut seq_vector: Vec<BigInt> = Vec::new();
    match seq_type {
        "fib" => {
            seq_vector.push(BigInt::from(0));
            seq_vector.push(BigInt::from(1));
        },
        "luc" => {
            seq_vector.push(BigInt::from(2));
            seq_vector.push(BigInt::from(1));
        },
        "brady" => {
            seq_vector.push(BigInt::from(2308));
            seq_vector.push(BigInt::from(4261));
        },
        _ => ()
    }

    //Create the sequences
    for i in 2..max_index + 1 {
        let new_seq = &seq_vector[i as usize - 2] + &seq_vector[i as usize - 1];
        seq_vector.push(new_seq);
    }

    seq_vector
}

//Recaman's Sequence
fn reca(index_string: Vec<String>) -> Vec<BigInt> {
    //Set the Recaman vector
    let mut rec_vector: Vec<BigInt> = vec![BigInt::from(0)];

    //Parse the vector and get max if possible
    let mut index_vector: Vec<usize> = Vec::new();
    for i in index_string {
        match i.trim().parse::<usize>() {
            Ok(x) => index_vector.push(x),
            Err(_) => break
        }
    }

    let mut max_index: usize = 0;
    if index_vector.len() != 0 {
        max_index = *index_vector.iter().max().unwrap();
    }

    //Generate the sequence to the highest number
    for n in 1..max_index + 1 {
        let mut next: BigInt = &rec_vector[n as usize - 1] - n;
        if next < BigInt::from(0) || rec_vector.contains(&next) {
            next = &rec_vector[n as usize - 1] + n;
        }

        rec_vector.push(next);
    }

    rec_vector
}

//Output the sequences
fn output_seq(seq_type: &str, index_string: Vec<String>, seq: Vec<BigInt>) {
    //Parse the indices into a new vector
    let mut i: Vec<usize> = Vec::new();
    for index in index_string {
        //Attempt to parse, exit the function if fail
        match index.trim().parse::<usize>() {
            Ok(x) => i.push(x),
            Err(_) => {
                println!("ERROR: You must enter positive integers only!");
                return;
            }
        }
    }

    //Display the appropriate text for the sequence command
    let mut index_offset: usize = 0;
    match seq_type {
        "fib" => {
            println!("---------------------------------------------");
            println!(">> Fibonacci Numbers <<");
            println!("---------------------------------------------");
        },
        "luc" => {
            println!("---------------------------------------------");
            println!(">> Lucas Numbers <<");
            println!("---------------------------------------------");
        },
        "brady" => {
            println!("---------------------------------------------");
            println!(">> Brady Numbers <<");
            println!("---------------------------------------------");
            index_offset = 1;
        },
        "reca" => {
            println!("---------------------------------------------");
            println!(">> Recaman Numbers <<");
            println!("---------------------------------------------");
        }
        _ => ()
    }

    //Print out the sequences
    if index_offset == 0 {
        for index in i {
            println!("{} - {}", index, seq[index]);
        }
    } else {
        for index in i {
            if index == 0 {
                println!("ERROR: {} is an invalid index", index);
                continue;
            }
            println!("{} - {}", index, seq[index - index_offset]);
        }
    }

    //Separator
    println!("---------------------------------------------");

    //pause
    pause();
}

//Pause the program for user input, then clear
fn pause() {
    let mut x = io::stdin();

    println!("Press any key to continue...");
    let _ = x.read(&mut [0u8]).unwrap();

    println!("{}", clear::BeforeCursor);
}

fn main() {
    loop {
        //Print the thing for the beginning of the loop
        println!("---------------------------------------------");
        println!(">> Enter a command <<");
        println!("---------------------------------------------");

        //Get the userinput and split it into a String vector
        let mut usercommand: Vec<String> = getinput().split_whitespace()
            .map(|x| x.to_string())
            .collect();

        //Get the sequence type/command and remove it from the input
        let seq_type = usercommand[0].clone();
        usercommand.remove(0);

        //Do the thing depending on the sequence type/command
        match seq_type.trim() {
            "fib" => output_seq("fib", usercommand.clone(), add_prev_two("fib", usercommand)),
            "luc" => output_seq("luc", usercommand.clone(), add_prev_two("luc", usercommand)),
            "brady" => output_seq("brady", usercommand.clone(), add_prev_two("brady", usercommand)),
            "reca" => output_seq("reca", usercommand.clone(), reca(usercommand)),
            "exit" => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!(">> ERROR: Invalid command!");
            }
        }
    }
}
