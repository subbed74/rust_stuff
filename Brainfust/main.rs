use std::io;
use std::fs::File;
use std::io::Read;

//Get input from the user
fn getinput() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput)
        .expect("Failed to read line");
    userinput
}

//Defines the  Brainfuck interpreter
struct Brainfuck {
    memory: [u32; 500],
    pointer: usize,
    skip: u8,
    code_pointer: usize,
    code_loop_begin: Vec<usize>
}

//Interprets the Brainfuck commands
impl Brainfuck {
    //Handles +/- operations
    fn inc_dec(&mut self, command: &str) {
        if self.skip == 0 {
            match command {
                "inc" => self.memory[self.pointer] += 1, // +
                "dec" => self.memory[self.pointer] -= 1, // -
                _ => {}
            }
        }
    }

    //Handles </> operations
    fn move_pointer(&mut self, direction: &str) {
        if self.skip == 0 {
            match direction {
                "left" => self.pointer -= 1, // <
                "right" => self.pointer += 1, // >
                _ => {}
            }
        }
    }

    //Handles ,/. operations
    fn console_io(&mut self, command: &str) {
        if self.skip == 0 {
            match command {
                "output" => print!("{}", (self.memory[self.pointer] as u8) as char), // .
                "input" => { // ,
                    let userinput: Vec<char> = getinput().chars()
                        .collect();
                    self.memory[self.pointer] = userinput[0] as u32;
                },
                _ => {}
            }
        }
    }

    //Handles looping with [/]
    fn loop_handle(&mut self, handle: &str) {
        match handle {
            "begin" => { // [
                if self.memory[self.pointer] != 0 {
                    self.code_loop_begin.push(self.code_pointer);
                } else {
                    self.skip += 1;
                }
            },
            "end" => { // ]
                if self.memory[self.pointer] != 0 {
                    match self.code_loop_begin.last() {
                        Some(&x) => self.code_pointer = x,
                        None => {}
                    }
                } else {
                    if self.skip == 0 {
                        self.code_loop_begin.pop();
                    } else {
                        self.skip -= 1;
                    }
                }
            },
            _ => {}
        }
    }
}

fn main() {
    //Read Brainfuck code from main.bf
    let mut bf_code: Vec<u8> = Vec::new();
    let mut f = File::open("main.bf").expect("Failed to open file!");
    f.read_to_end(&mut bf_code).expect("Failed to read the file!");

    //Setup the Brainfuck interpreter
    let mut bf_info = Brainfuck {
        memory: [0; 500],
        pointer: 0,
        skip: 0,
        code_pointer: 0,
        code_loop_begin: Vec::new()
    };

    //Loop through the code and match every character to an operation
    loop {
        match bf_code[bf_info.code_pointer] {
            43 => bf_info.inc_dec("inc"), // +
            45 => bf_info.inc_dec("dec"), // -
            62 => bf_info.move_pointer("right"), // >
            60 => bf_info.move_pointer("left"), // <
            46 => bf_info.console_io("output"), // .
            44 => bf_info.console_io("input"), // ,
            91 => bf_info.loop_handle("begin"), // [
            93 => bf_info.loop_handle("end"),  // ]
            _ => {}
        }

        //Increment the code pointer
        bf_info.code_pointer += 1;

        //If the pointer is greater than the length of the code, terminate the loop
        if bf_info.code_pointer >= bf_code.len() {
            break;
        }
    }
}
