#[allow(unused_imports)]
use std::io::{self, Write, BufRead};


fn command_execute(command: &str) -> &str {
//create pattern to match string to different commands
//split the commadn its a vector of string splices
//let command: Vec<&str> = s.split(' ').collect()
// use match pattern with copmmand[0]
// return the command output
    let words: Vec<&str> = command.split(' ').collect();

    match words.first().copied() {
        Some("exit") => {
            return "exit"
        }
        Some("echo") => {
            println!("{}", &words[1..].join(" "));
            return "echo"
        }
        Some(_) => {
            println!("{}: command not found", command);
            return "invalid"
        }
        None => {
            println!("Empty command");
            return "invalid"
        }

    }
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    //stdout variable needs to be mutable because flush modifies the internal state of stdout
    
    let mut stdout: io::Stdout = io::stdout();
    
    //the input stream from the keyboard
    let stdin: io::Stdin = io::stdin();
    
    print!("$ ");
    stdout.flush().unwrap();

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    print!("$ ");
                    stdout.flush().unwrap();
                    continue;
                }
                let command_result = command_execute(&line);
                if command_result == "exit" {
                    break;
                }
                print!("$ ");
                stdout.flush().unwrap();
            }
            Err(_) => {
                println!("Error reading input");
            }
        }


        // match line {
        //     Ok(line) => {
        //         if line.trim().is_empty() {
        //             print!("$ ");
        //             stdout.flush().unwrap();
        //             continue;
        //         }
        //         if line.trim() == "exit" {
        //             break;
        //         } else {
        //             println!("{}: command not found", line);
        //         }
        //         print!("$ ");
        //         stdout.flush().unwrap();
        //     }
        //     Err(_) => {
        //         println!("Error reading input");
        //     }
        // }
        

    }




    
    
}
