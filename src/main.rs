use std::env;
use std::path::Path;
use std::io;
use std::io::Write;
use std::process::Command;

fn change_dir(args: &Vec<&str>) {
  let path_buf = env::home_dir().unwrap();
  let path = match args.len() {
    0 => path_buf.as_path(),
    _ => Path::new(&args[0]),
  };
  env::set_current_dir(&path).is_ok();
  return;
}

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

    match tokens[0] {
      "cd" => { return change_dir(&args); }
      &_   => {}
    }

    let mut subproc = Command::new(tokens[0]);
    let mut subproc_with_args = subproc.args(&args);

    match subproc_with_args.spawn() {
      Ok(mut child) => {
        if no_wait == false {
          let _ = child.wait();
        }
      }
      Err(err) => {
        println!("Failed to execute {}: {}", tokens[0], err);
        return;
      }
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
