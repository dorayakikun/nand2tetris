use crate::arithmetic_command::ArithmeticCommand;
use crate::command::Command;
use crate::segment::Segment;
use anyhow::anyhow;
use anyhow::{Context, Result};
use std::io::{self, BufRead};
use std::{fs::File, path::PathBuf};
pub fn parse(file_name: &PathBuf) -> Result<Vec<Command>> {
    let lines = read_lines(file_name)?;
    let mut commands: Vec<Command> = vec![];
    for line in lines {
        let line = line?;
        let line = line.trim().to_string();
        let line = match line.find("//") {
            Some(i) => line[0..i].to_string(),
            None => line,
        };

        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let command = make_command(tokens)?;
        commands.push(command);
    }
    Ok(commands)
}

fn read_lines(file_name: &PathBuf) -> Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn make_command(tokens: Vec<&str>) -> Result<Command> {
    match tokens[0] {
        "add" => Ok(Command::Arithmetic(ArithmeticCommand::Add)),
        "sub" => Ok(Command::Arithmetic(ArithmeticCommand::Add)),
        "neg" => Ok(Command::Arithmetic(ArithmeticCommand::Neg)),
        "eq" => Ok(Command::Arithmetic(ArithmeticCommand::Eq)),
        "gt" => Ok(Command::Arithmetic(ArithmeticCommand::Gt)),
        "lt" => Ok(Command::Arithmetic(ArithmeticCommand::Lt)),
        "and" => Ok(Command::Arithmetic(ArithmeticCommand::And)),
        "or" => Ok(Command::Arithmetic(ArithmeticCommand::Or)),
        "not" => Ok(Command::Arithmetic(ArithmeticCommand::Not)),
        "push" => {
            if tokens.len() != 3 {
                return Err(anyhow!(
                    "the length of tokens is invalid. expected: 3, actual: {}",
                    tokens.len()
                ));
            }
            let value: i32 = tokens[2]
                .parse()
                .with_context(|| format!("arg2 is not a number: {}", tokens[2]))?;
            match tokens[1] {
                "argument" => Ok(Command::Push(Segment::Argument, value)),
                "local" => Ok(Command::Push(Segment::Local, value)),
                "static" => Ok(Command::Push(Segment::Static, value)),
                "constant" => Ok(Command::Push(Segment::Constant, value)),
                "this" => Ok(Command::Push(Segment::This, value)),
                "that" => Ok(Command::Push(Segment::That, value)),
                "pointer" => Ok(Command::Push(Segment::Pointer, value)),
                "temp" => Ok(Command::Push(Segment::Temp, value)),
                _ => Err(anyhow!("invalid segment found: {}", tokens[1])),
            }
        }
        "pop" => {
            if tokens.len() != 3 {
                return Err(anyhow!(
                    "the length of tokens is invalid. expected: 3, actual: {}",
                    tokens.len()
                ));
            }
            let value: i32 = tokens[2]
                .parse()
                .with_context(|| format!("arg2 is not a number: {}", tokens[2]))?;
            match tokens[1] {
                "argument" => Ok(Command::Pop(Segment::Argument, value)),
                "local" => Ok(Command::Pop(Segment::Local, value)),
                "static" => Ok(Command::Pop(Segment::Static, value)),
                "constant" => Ok(Command::Pop(Segment::Constant, value)),
                "this" => Ok(Command::Pop(Segment::This, value)),
                "that" => Ok(Command::Pop(Segment::That, value)),
                "pointer" => Ok(Command::Pop(Segment::Pointer, value)),
                "temp" => Ok(Command::Pop(Segment::Temp, value)),
                _ => Err(anyhow!("invalid segment found: {}", tokens[1])),
            }
        }
        _ => Err(anyhow!("invalid command found: {}", tokens[0])),
    }
}
