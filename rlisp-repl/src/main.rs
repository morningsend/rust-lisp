use std::io;
use std::io::Write;
use std::process;

fn clean_input(s: &str) -> &str {
    return s.trim();
}

//extern crate rlisp;

use rlisp::interpreter::{BasicInterpreter, Interpreter};

fn main() {
    let prompt = String::from("rlisp-user:> ");
    let mut line = String::new();
    let mut interp = BasicInterpreter::new();

    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("error reading source");

        let code = clean_input(&line);

        if code == "quit" {
            println!("bye!");
            process::exit(0);
        }

        match interp.eval(&line) {
            Ok(output) => {
                println!("{}", output.output);
            }
            Err(e) => {
                eprintln!("error evaluating: {}", e);
            }
        }
    }
}
