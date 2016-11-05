use std::io;
use std::io::Write;
use std::process::Command;

fn process(buffer: &String) {

    let mut tokens: Vec<&str> = buffer.split_whitespace().collect();
    let buffer_len = tokens.len();

    if buffer_len == 0 {
        return;
    }

    let no_wait = tokens[buffer_len - 1] == "&";
    let mut args = tokens.split_off(1);

    if no_wait {
        args.truncate(buffer_len - 2);
    }

    let mut subproc = Command::new(tokens[0]);
    let mut subproc_with_args = subproc.args(&args);
    let mut child = subproc_with_args.spawn().expect("Failed to execute");

    if no_wait == false {
        let _ = child.wait();
    }

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
