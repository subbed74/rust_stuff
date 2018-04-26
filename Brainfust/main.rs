use std::io;
use std::fs::File;
use std::io::Read;
use std::env;

//Get input from the user
fn getinput() -> String {
    let mut userinput = String::new();
    io::stdin().read_line(&mut userinput)
        .expect("Failed to read line");
    userinput
}

//Defines the  Brainfuck interpreter
struct Brainfuck {
    memory: [u32; 30000],
    pointer: usize,
    skip: u8,
    code_pointer: usize,
    code_loop_begin: Vec<usize>,
    input_vector: Vec<char>
}

//Interprets the Brainfuck commands
impl Brainfuck {
    //Handles +/- operations
    fn inc_dec(&mut self, command: &str) {
        if self.skip == 0 {
            match command {
                "inc" => { // +
                    match self.memory[self.pointer].checked_add(1) {
                        Some(x) => self.memory[self.pointer] = x,
                        None => self.memory[self.pointer] = 0
                    }
                },
                "dec" => { // -
                    match self.memory[self.pointer].checked_sub(1) {
                        Some(x) => self.memory[self.pointer] = x,
                        None => self.memory[self.pointer] = u32::max_value()
                    }
                },
                _ => {}
            }
        }
    }

    //Handles </> operations
    fn move_pointer(&mut self, direction: &str) {
        if self.skip == 0 {
            match direction {
                "left" => { // <
                    match self.pointer.checked_sub(1) {
                        Some(x) => self.pointer = x,
                        None => self.pointer = 1999,
                    }
                },
                "right" => { // >
                    self.pointer += 1;

                    if self.pointer >= 2000 { self.pointer = 0 }
                },
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
                    //If the input_vector is empty, store the entire input in it.
                    if self.input_vector.is_empty() {
                        self.input_vector = getinput().chars().collect();
                        self.memory[self.pointer] = self.input_vector[0] as u32;
                        self.input_vector.remove(0);
                    } else {
                        self.memory[self.pointer] = self.input_vector[0] as u32;
                        self.input_vector.remove(0);
                    }
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
    //Set the working director to the same directory as the executable
    let mut exec_dir = env::current_exe().expect("Heck.");
    exec_dir.pop();
    env::set_current_dir(exec_dir).expect(".kceH");

    //Read Brainfuck code from main.bf
    let mut bf_code: Vec<u8> = Vec::new();
    let mut f = File::open("main.bf").expect("Failed to open file!");
    f.read_to_end(&mut bf_code).expect("Failed to read the file!");

    //Setup the Brainfuck interpreter
    let mut bf_info = Brainfuck {
        memory: [0; 30000],
        pointer: 0,
        skip: 0,
        code_pointer: 0,
        code_loop_begin: Vec::new(),
        input_vector: Vec::new()
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
