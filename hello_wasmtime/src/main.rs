use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn do_copy(source: &str, destination: &str) -> Result<(), String> {
    let mut source_file =
        File::open(source).map_err(|err| format!("error opening source {}: {}", source, err))?;

    let mut contents = Vec::new();
    source_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("Error reading: {}", err))?;

    let mut destination_file = File::create(destination)
        .map_err(|err| format!("Error creating destination {}: {}", destination, err))?;

    destination_file
        .write_all(&contents)
        .map_err(|err| format!("Write error: {}", err))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("Usage: {} <source> <destination>", program);
        return ;
    }

    let l = args.len();

    if let Err(err) = do_copy(&args[l - 2], &args[l - 1]) {
        eprintln!("{}", err);
    }
}
