use crate::command::Command;

pub fn parse(file_name: &str) -> Result<Vec<Command>, std::io::Error> {
    Ok(vec![Command::Arithmetic(crate::arithmetic_command::ArithmeticCommand::Add)])
}
