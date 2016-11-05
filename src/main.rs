use std::io;
use std::io::Write;
use std::process::Command;

fn process(buffer: &String) {

    let mut tokens: Vec<&str> = buffer.split_whitespace().collect();
    let mut subproc = Command::new(tokens[0]);
    let mut subproc_with_args = subproc.args(&tokens.split_off(1));
    let mut child = subproc_with_args.spawn().expect("Failed to execute");
    let _ = child.wait();
    let _ = io::stdout().flush();
}

fn main() {
    let mut buffer = String::new();
    loop {
        print!("$ ");
        let _ = io::stdout().flush();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                process(&buffer);
            }
            Err(err) => {
                println!("Read error: {}", err);
            }
        }
        buffer.truncate(0);
    }
}
