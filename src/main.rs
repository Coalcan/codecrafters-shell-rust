#[allow(unused_imports)]
use std::io::{self, Write, BufRead};


// fn is_command(command: &str) -> &str {
//     if command == "exit" {
//         "valid"
//     } else {
//         "invalid"
//     }
// }

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
                if line.trim() == "exit" {
                    break;
                } else {
                    println!("{}: command not found", line);
                }
                print!("$ ");
                stdout.flush().unwrap();
            }
            Err(_) => {
                println!("Error reading input");
            }
        }
        //handle empty, no command, not working, improperly handling empty variable
        // else {
        //     println!("Empty command");
        // }
        

    }




    
    
}
