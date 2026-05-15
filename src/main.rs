#[allow(unused_imports)]
use std::io::{self, Write};

#[allow(unused_imports)]
use bytes::buf;

const BUILT_IN_COMMANDS: [&str; 3] = ["type", "echo", "exit"];
fn main() {
    //TODO: Uncomment the code below to pass the first stage
    // print!("$ ");
    // io::stdout().flush().unwrap();
    // let mut command = String::new();
    // io::stdin().read_line(&mut command).unwrap();
    // println!("{}: command not found", command.trim());

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        if command.eq("exit"){
            break;
        }
        else if command.starts_with("echo"){
            println!("{}", &command[5..])
        }
        else if command.starts_with("type"){
            let builtin = &command[5..];
            if  BUILT_IN_COMMANDS.contains(&builtin) {
                println!("{} is a shell builtin", builtin);
            }
            else {
                println!("{}: not found", builtin);
            }
        }
        else{
            println!("{}: command not found", command);
        }
    }

}
