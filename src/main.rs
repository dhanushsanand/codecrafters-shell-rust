#[allow(unused_imports)]
use std::io::{self, Write};

#[allow(unused_imports)]
use bytes::buf;

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
        if command.trim().eq("exit"){
            break;
        }
        if command.trim().starts_with("echo"){
            print!("{}", &command[5..])
        }else{
            println!("{}: command not found", command.trim());
        }
    }

}
