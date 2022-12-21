use console::Term;
use std::{env, usize};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_content = std::fs::read_to_string(file_path).expect("Failed to read file");
    let file_content_chars = file_content.chars().collect::<Vec<char>>();
    let mut memory_cells = [0u8; 30000];
    let mut memory_index: usize = 0;
    let last_memory_index = memory_cells.len() - 1;
    let term = Term::stdout();
    let mut b = 0;
    let mut i = 0;
    while i < file_content_chars.len() {
        let e = file_content_chars[i];
        match e {
            '+' => {
                if memory_cells[memory_index] == 255 {
                    memory_cells[memory_index] = 0;
                    i += 1;
                } else {
                    memory_cells[memory_index] += 1;
                    i += 1;
                }
            }
            '-' => {
                if memory_cells[memory_index] == 0 {
                    memory_cells[memory_index] = 255;
                    i += 1;
                } else if memory_cells[memory_index] > 0 {
                    memory_cells[memory_index] -= 1;
                    i += 1;
                }
            }
            '>' => {
                if memory_index == 29999 {
                    memory_index = 0;
                    i += 1;
                } else {
                    memory_index += 1;
                    i += 1;
                }
            }
            '<' => {
                if memory_index == 0 {
                    memory_index = last_memory_index;
                    i += 1;
                } else if memory_index > 0 {
                    memory_index -= 1;
                    i += 1;
                }
            }
            '.' => {
                print!("{}", memory_cells[memory_index]);
                i += 1;
            }
            ',' => {
                memory_cells[memory_index] = term.read_char().unwrap() as u8;
                i += 1;
            }
            '[' => {
                if memory_cells[memory_index] != 0 {
                    i += 1;
                    continue;
                }
                b += 1;
                while b != 0 {
                    i += 1;
                    match file_content_chars[i] {
                        '[' => b += 1,
                        ']' => b -= 1,
                        _ => {
                            continue;
                        }
                    }
                }
                i += 1;
            }
            ']' => {
                if memory_cells[memory_index] == 0 {
                    i += 1;
                    continue;
                }
                b += 1;
                while b != 0 {
                    i = i - 1;
                    match file_content.chars().collect::<Vec<char>>()[i] {
                        '[' => b -= 1,
                        ']' => b += 1,
                        _ => {
                            continue;
                        }
                    }
                }
            }
            _ => {
                i += 1;
                continue;
            }
        }
    }
}
