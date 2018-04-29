use std::io;

//Get userinput
fn getinput() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput)
        .expect("Failed to read line");
    userinput
}

//Format the tone if it's 10/11 to T/E for alignment
fn format_pitch(tone: i8) -> String {
    match tone {
        10 => "T".to_string(),
        11 => "E".to_string(),
        x => x.to_string(),
    }
}

//Make sure the tone calculations stay between 0-11 inclusive
fn check_tone_range(tone: i8) -> i8 {
    if tone > 11 {
        tone - 12
    } else if tone < 0 {
        tone + 12
    } else {
        tone
    }
}

fn main() {
    //Get P0 from the user
    println!("Enter the first prime row (use numbers 0-11 to represent pitch class):");
    let main_prime: Vec<i8> = getinput().split_whitespace()
        .map(|x| x.parse::<i8>().unwrap())
        .collect();

    //I like to separate things
    println!("--------------------------------------------");

    //Calculate interval differences
    let mut intervals: Vec<i8> = Vec::new();
    for i in 0..11 {
        intervals.push(main_prime[i as usize] - main_prime[i as usize + 1]);
    }

    //Calculate first inversion column
    let mut main_inversion: Vec<i8> = vec![main_prime[0]];
    for i in 0..11 {
        let mut next_tone = check_tone_range((main_inversion[i as usize] - (-1 * intervals[i as usize])) % 12);
        main_inversion.push(next_tone);
    }

    //Print out the given row
    let tone_print: String = format_pitch(main_prime[0]);
    print!("\nP({}) - ", tone_print);
    for tone in main_prime {
        let tone_print: String = format_pitch(tone);
        print!("{} ", tone_print);
    }
    println!("- R({})", tone_print);

    //Calculate and print out the rest of the rows
    for i in 1..12 {
        //Calculate the row
        let mut next_row: Vec<i8> = vec![main_inversion[i as usize]];
        for x in 0..11 {
            let mut next_tone: i8 = check_tone_range((next_row[x as usize] + (-1 * intervals[x as usize])) % 12);
            next_row.push(next_tone);
        }

        //Print the row
        let tone_print: String = format_pitch(next_row[0]);
        print!("P({}) - ", tone_print);
        for tone in next_row {
            let tone_print: String = format_pitch(tone);
            print!("{} ", tone_print);
        }
        println!("- R({})", tone_print);
    }

    //I like to separate things
    println!("\n--------------------------------------------");
}
