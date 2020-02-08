extern crate clap;

use clap::{Arg, App};

fn main() {
    let app = App::new("rlisp-bin")
        .version("0.0.1")
        .author("Zaiyang Li")
        .about("Basic LISP intepreter written in Rust")
        .arg(
            Arg::with_name("file")
                .index(1)
                .required(true)
        );
    
    let matches = app.get_matches();
    let source_file = matches.value_of("file").expect("must specify a file");
    println!("running file {}", source_file);
    println!("Hello, world!");
}
