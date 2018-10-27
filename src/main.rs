use std::io::{self, BufRead, Write};

mod parser;
mod process;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("# ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut buf = String::new();

        stdin.lock().read_line(&mut buf).expect("Could not read from stdin");

        if buf.len() == 0 {
            println!();
            break;
        }
        //println!("{:?}", buf.split_whitespace());
        process::receive_command(&mut buf.split_whitespace().collect());
    }
}
