use std::process::{Command, Stdio};

pub fn receive_command(vector: &mut Vec<&str>)  {
    let command  = vector.get(0).unwrap().to_string();
    let getting_arguments: Vec<_> = vector.drain(1..).collect();
    
    if let Err(_) = Command::new(command)
        .args(&getting_arguments)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output() {
            eprintln!("Command not found");
        }
}

