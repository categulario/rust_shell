use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
extern crate dirs;


#[derive(Debug)]
pub struct Process<'a> {
    command: &'a String,
    arguments: Vec<&'a str>,
}

pub fn receive_command(vector: &mut Vec<&str>) {
    let process = Process { 
        command  : &vector.get(0).unwrap().to_string(),
        arguments : vector.drain(1..).collect(),
    };

   if process.command == "cd"{
       if process.arguments.len() == 0{
           println!("Es vacio");
           //let path = env::current_dir();
           let current_home = dirs::home_dir();
           println!("path: {:?}", current_home);
           match current_home{
                Some(d) =>    { 
                   //println!("The current directory is {}", d.display());
                   //println!("The current directory is {:?}", d.to_str());
                   //let x: () = d;
                   //println!("The current directory is {:?}", x);
                  // execute_cd_process(&d.to_str());
                   let root = Path::new(&d);
                   let _changed_dir = env::set_current_dir(&root).is_ok();
                },
                None => {
                   println!("eror"); 
                },
           }
            
       }else{
            let cd = &process.arguments.get(0).unwrap().to_string();
            execute_cd_process(cd.to_string());
       }
       
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
    //println!("The current directory is {}", root.display());
}

#[test]
fn execute_a_single_command(){
    let mut process = vec!["echo 'hello'"];
    assert_eq!("hello".trim(), receive_command(&mut process) );
}

