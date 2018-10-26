use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!("# ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut buf = String::new();

        stdin.lock().read_line(&mut buf);

        if buf.len() == 0 {
            println!();
            break;
        }

        println!("{:?} {}", buf, buf.len());
    }
}
