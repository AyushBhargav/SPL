pub mod lexer;

use std::env;
use std::process;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        2 => run_file(&args[1]),
        1 => run_repl(),
        _ => {
            println!("Usage spl [main script]");
            process::exit(64)
        }
    }
}

fn run_file(file_path: &String) {
    let mut file = File::open(file_path).expect("File not found.");

    let mut content_buffer = String::new();
    file.read_to_string(&mut content_buffer).expect("File is not parsable.");

    run(content_buffer);
}

fn run_repl() {
    println!("REPL shell for SPL.");
    loop {
        print!("> ");
        io::stdout().flush().expect("Output error.");
        let mut code_line = String::new();
        io::stdin().read_line(&mut code_line).expect("Can't take input. Aborting.");
        run(code_line);
    }
}

fn run(source: String) {
    let source_trimmed = source.trim();
    for (i, token) in source_trimmed.chars().enumerate() {
        println!("{}: {}", i, token);
    } 
}