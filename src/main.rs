#[allow(unused_imports)]
use std::io::{self, Write};

use std::{env, fs, os::unix::fs::PermissionsExt, path};

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
    let path_variable = env::var("PATH").unwrap_or_default();
    let paths_list: Vec<path::PathBuf> = env::split_paths(&path_variable).collect();
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
            let mut flag = false;
            for directory in &paths_list {
                match fs::read_dir(directory) {
                    Ok(entries) => {
                        for entry in entries.flatten(){
                            if let Ok(file_name) = entry.file_name().into_string(){
                                if file_name == builtin{
                                    if fs::metadata(directory.join(&file_name)).map(|m| m.permissions().mode() & 0o111 !=0).unwrap_or(false){
                                        println!("{} is {}", builtin, directory.join(file_name).display());
                                        flag = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(_e) =>{
                        //println!("{}: not found in {}", builtin, directory.display());
                    }
                }
                if flag{
                    break;
                }
                // if let Ok(entries) = fs::read_dir(directory) {
                //     for entry in entries.flatten() {
                //         if let Ok(file_name) = entry.file_name().into_string() {
                //             if file_name == builtin {
                //                 println!("{} is {}", builtin, entry.path().display());
                //                 break;
                //             }
                //         }
                //     }
                // }
            }
            if !flag{
                println!("{}: not found", builtin);
            }
        }
        else{
            println!("{}: not found", command);
        }
    }

}
