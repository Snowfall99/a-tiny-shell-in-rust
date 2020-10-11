use std::io::{stdout, Write, stdin};
use std::process::{Command, Stdio, Child};
use std::path::Path;
use std::env;

fn main() {
    loop {
        print!(">");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;
        if let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "quit" => {
                    println!("quit the shell, bye.");
                    return;
                },
                "cd" => {
                    let new_dirs = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dirs);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                command => {
                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child|Stdio::from(output.stdout.unwrap())
                        );
                    
                    let stdout = if commands.peek().is_some() {
                        // There is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // There are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
            
            if let Some (mut final_command) = previous_command {
                // block until the final command has finished
                final_command.wait().unwrap();
            }
        }
        
        
    }

}