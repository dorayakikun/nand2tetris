use crate::segment::Segment;
pub enum Command {
    Push(Segment, i32),
}
