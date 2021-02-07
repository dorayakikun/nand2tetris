use crate::arithmetic_command::ArithmeticCommand;
use crate::segment::Segment;
#[derive(Debug)]
pub enum Command {
    Arithmetic(ArithmeticCommand),
    Push(Segment, i32),
    Pop(Segment, i32),
}
