use crate::arithmetic_command::ArithmeticCommand;
use crate::segment::Segment;
#[derive(Debug)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    Push(Segment, i32),
    Pop(Segment, i32),
    Label(String),
    GoTo(String),
    IfGoTo(String),
    Function(String, i32),
    Call(String, i32),
    Return,
}
