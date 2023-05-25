use std::{fs::read_to_string, env, io::{stdin, Read}};

fn main() {
    let input_str: String;

    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && args[1] == "-i" {
        let file_path = &args[2];
        let contents = read_to_string(file_path).unwrap();

        input_str = contents;
    } else if args.len() == 2 {
        input_str = args[1].clone();
    } else {
        panic!("Invalid arguments provided. Expected -i <file> or <brainfuck code>");
    }

    let input: Vec<char> = input_str.chars().collect();
    let mut current_instruction = 0;

    let mut output = vec![0u8; 30000];
    let mut pointer = 0;

    while current_instruction < input.len() {
        let instruction = input.get(current_instruction).unwrap();
        let value = *output.get(pointer).unwrap();
        match instruction {
            '+' => {
                output[pointer] = value.wrapping_add(1);
                current_instruction += 1;
            }
            '-' => {
                output[pointer] = value.wrapping_sub(1);
                current_instruction += 1;
            }
            '>' => {
                pointer += 1;

                if pointer >= output.len() {
                    pointer = 0;
                }

                current_instruction += 1;
            }
            '<' => {
                if pointer == 0 {
                    pointer = output.len() - 1;
                } else {
                    pointer -= 1;
                }

                current_instruction += 1;
            }
            '.' => {
                print!("{}", value as char);

                current_instruction += 1;
            }
            '[' => {
                if value == 0 {
                    let mut inner_loops = 0;
                    let mut i = current_instruction + 1;
                    loop {
                        if i >= input.len() {
                            panic!("No matching ] found for [");
                        }

                        match input.get(i).unwrap() {
                            &'[' => {
                                inner_loops += 1;
                            }
                            &']' => {
                                if inner_loops == 0 {
                                    break;
                                }

                                inner_loops -= 1;
                            }
                            _ => {}
                        }

                        i += 1;
                    }

                    current_instruction = i + 1;
                } else {
                    current_instruction += 1;
                }
            }
            ']' => {
                if value == 0 {
                    current_instruction += 1;
                } else {
                    let mut inner_loops = 0;
                    let mut i = current_instruction - 1;
                    loop {
                        if i <= 0 {
                            panic!("No matching [ found for ]");
                        }

                        match input.get(i).unwrap() {
                            &']' => {
                                inner_loops += 1;
                            }
                            &'[' => {
                                if inner_loops == 0 {
                                    break;
                                }

                                inner_loops -= 1;
                            }
                            _ => {}
                        }

                        i -= 1;
                    }

                    current_instruction = i + 1;
                }
            }
            ',' => {
                let mut buffer = [0u8; 1];
                stdin().read_exact(&mut buffer).unwrap();
                output[pointer] = buffer[0];

                current_instruction += 1;
            }
            _ => {}
        }
    }
}
