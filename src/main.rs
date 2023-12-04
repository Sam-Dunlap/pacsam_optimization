use std::{io, process};

fn main() {
    println!("File Path >");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("could not parse file path");
    file_path = file_path.trim().into();
    if let Err(e) = pacsam_optimization::run(file_path) {
        eprintln!("Problem: {e}");
        process::exit(1);
    }
}
