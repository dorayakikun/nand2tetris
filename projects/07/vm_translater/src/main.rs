use std::env;
use std::fs::File;
use std::io::{self, BufRead};

mod parser;
mod code_writer;

fn main() -> Result<(), io::Error>{
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    if args.len() < 2 {
        println!("please enter file name");
        return Ok(());
    }

    let lines = read_lines(&args[1])?;

    for line in lines {
        let line = line?;
        if !is_valid_command(&line) {
            continue;
        }
        println!("{}", line)
    }
    Ok(())
}

fn read_lines(file_name: &str) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_valid_command(line: &str) -> bool {
    if line.trim().is_empty() ||  line.trim().starts_with("//"){
        return false;
    }
    true
}
