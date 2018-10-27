use std::process::{Command, Stdio};

#[derive(Debug)]
struct Process<'a> {
    command: String,
    arguments: Vec<&'a str>,
}


pub fn receive_command(vector: &mut Vec<&str>) {
    let process = Process { 
        command  : vector.get(0).unwrap().to_string(),
        arguments : vector.drain(1..).collect(),
    };

    if let Err(_) = Command::new(process.command)
        .args(process.arguments)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output() {
            eprintln!("Command not found");
        }
}

