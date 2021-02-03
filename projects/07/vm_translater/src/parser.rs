use crate::command::Command;
use std::{fs::File, ops::Index};
use std::io::{self, BufRead};
pub fn parse(file_name: &str) -> Result<Vec<Command>, std::io::Error> {
    let lines = read_lines(file_name)?;
    for line in lines {
        let line = line?;
        if line.starts_with("//") {
            continue;
        }
        if let Some(index) = line.find("//") {
            
        }
    }
    Ok(vec![Command::Arithmetic(crate::arithmetic_command::ArithmeticCommand::Add)])
}

fn read_lines(file_name: &str) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}