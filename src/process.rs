use std::process::{Command, Stdio};
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Process<'a> {
    command: String,
    arguments: Vec<&'a str>,
}

pub fn receive_command(vector: &mut Vec<&str>) {
    let process = Process { 
        command  : vector.get(0).unwrap().to_string(),
        arguments : vector.drain(1..).collect(),
    };

   if process.command == "cd"{
       let cd = &process.arguments.get(0).unwrap().to_string();
        execute_cd_process(cd.to_string());
    }else if process.command == "exit"{
       println!("Es exit");
    }else {
       execute_process(process);
    }
}

pub fn execute_process(process: Process){
    if let Err(_) = Command::new(process.command)
        .args(process.arguments)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output() {
            eprintln!("Command not found");
        }
}

pub fn execute_cd_process(cd: String){
    let root = Path::new(&cd);
    let _changed_dir = env::set_current_dir(&root).is_ok();
    println!("The current directory is {}", root.display());
}

#[test]
fn execute_a_single_command(){
    let mut process = vec!["echo 'hello'"];
    assert_eq!("hello".trim(), receive_command(&mut process) );
}

