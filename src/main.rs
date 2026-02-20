#[allow(unused_imports)]
use std::io::{self, Write, BufRead};
use std::collections::HashSet;
use std::sync::OnceLock;
use pathsearch::find_executable_in_path;
use std::process::Command;

//path environment variable for external commands


static SHELL_COMMANDS: OnceLock<HashSet<&str>> = OnceLock::new();

fn get_shell_commands() -> &'static HashSet<&'static str> {
    SHELL_COMMANDS.get_or_init(|| {
        HashSet::from([
            "exit",
            "echo",
            "type",
        ])
    })
}

//check for implicit copies in memory to optimize for memory usage and performance
//look into flame graphs to identify bottlenecks in the code and optimize them
fn command_execute(command: &str) -> std::io::Result<()>{

    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        Ok(())
    } 
    else {

        let output: std::process::Output = Command::new(parts[0])
            .args(&parts[1..])
            .output()?;

        print!("{}", String::from_utf8_lossy(&output.stdout));
        if !output.status.success() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }

    //execute the command and print the output
    //use std::process::Command to execute the command
    //capture the output and print it to the console
    //handle errors gracefully and print error messages to the console
}


fn command_validate(command: &str) -> &str {
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
        Some("type") => {
            //embed
            if let Some(command) = words.get(1) {

                if get_shell_commands().contains(command) {

                    println!("{} is a shell builtin", command);
                    return "type"

                } if let Some(path) = find_executable_in_path(command) {

                    println!("{} is {}", command, path.display());
                    return "type"

                }

                //add in search for external commands in the PATH environment variable
                //shell must go through every directory in PATH. For each directory:
                //Check if a file with the command name exists.
                //Check if the file has execute permissions.
                //If the file exists and has execute permissions, print <command> is <full_path> and stop.
                //If the file exists but lacks execute permissions, skip it and continue to the next directory.


                else {

                    println!("{}: not found", command);
                    return "invalid"

                }
            } else {

                println!("type: missing argument");
                return "invalid"

            }
        }
        Some(_) => {
            return "unknown"
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

                let command_result = command_validate(&line);


                //handle command results that require a change in the shell state.

                match command_result {
                    "exit" => {
                        break;
                    }
                    "unknown" => {
                        if let Err(_) = command_execute(&line) {
                            println!("{}: not found", line.split_whitespace().next().unwrap_or(""));
                        }
                    }
                    //pattern must be exaustive 
                    _ => {}
                }
                
                print!("$ ");
                stdout.flush().unwrap();
            }
            Err(_) => {
                println!("Error reading input");
            }
        }
        

    }

}