#[allow(unused_imports)]
use std::io::{self, Write, BufRead};


fn is_valid_command(command: &str) -> bool {
    if command == "goddamn anything" {
        false
    } else {
        false
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
        if let Ok(command) = line {
            if is_valid_command(&command) {
                //execute command
            } else {
                println!("{}: command not found", command);
                println!("Type: {}", std::any::type_name::<&str>());
            }

            print!("$ ");
            stdout.flush().unwrap();
        }
        //handle empty, no command, not working, improperly handling empty variable
        else {
            println!("Empty command");
        }
        

    }




    
    
}
