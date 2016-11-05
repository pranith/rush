use std::io;
use std::io::Write;

fn process(buffer: &String) {
    println!("Size of command {}", buffer.len());
}

fn main() {
    let mut buffer = String::new();
    loop {
        print!("$ ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                process(&buffer);
                println!("Read {} bytes: {}", n, buffer);
            }
            Err(err) => {
                println!("Read error: {}", err);
            }
        }
        buffer.truncate(0);
    }
}
