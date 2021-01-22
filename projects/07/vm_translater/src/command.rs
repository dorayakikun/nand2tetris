use crate::arithmetic_command::ArithmeticCommand;
use crate::segment::Segment;
pub enum Command {
    Arithmetic(ArithmeticCommand),
    Push(Segment, i32),
    Pop(Segment, i32),
    Label(String, String),
    Goto(String, String),
    IfGoto(String, String),
    DefineFunction(String, i32),
    CallFunction(String, i32),
    Return,
}
