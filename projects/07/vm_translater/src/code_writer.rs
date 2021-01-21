use crate::{command::Command, segment::Segment};

const PUSH_ONTO_STACK: &str = r#"// push onto stack
@SP
A=M
M=D
@SP
M=M+1"#;

pub fn write_code(file_name: &str, command: &Command) -> String {
    match command {
        Command::Push(segment, index) => write_code_push(file_name, segment, index),
    }
}

fn write_code_push(file_name: &str, segment: &Segment, index: &i32) -> String {
    match segment {
        Segment::Argument => {
            format!(
                "{}\n{}\n{}",
                write_load_value_into_d(index),
                write_load_value_offset_into_d("ARG"),
                PUSH_ONTO_STACK
            )
        }
        Segment::Local => {
            format!(
                "{}\n{}\n{}",
                write_load_value_into_d(index),
                write_load_value_offset_into_d("LCL"),
                PUSH_ONTO_STACK
            )
        }
        Segment::Static => {
            format!("@{}.{}\nD=M\n{}", file_name, index, PUSH_ONTO_STACK)
        }
        Segment::Constant => {
            format!("{}\n{}", write_load_value_into_d(index), PUSH_ONTO_STACK)
        }
        Segment::This => {
            format!(
                "{}\n{}\n{}",
                write_load_value_into_d(index),
                write_load_value_offset_into_d("THIS"),
                PUSH_ONTO_STACK
            )
        }
        Segment::That => {
            format!(
                "{}\n{}\n{}",
                write_load_value_into_d(index),
                write_load_value_offset_into_d("THAT"),
                PUSH_ONTO_STACK
            )
        }
        Segment::Pointer => {
            format!(
                "{}\n@{}\nA=D+A\nD=M\n{}",
                write_load_value_into_d(index),
                3,
                PUSH_ONTO_STACK
            )
        }
        Segment::Temp => {
            format!(
                "{}\n@{}\nA=D+A\nD=M\n{}",
                write_load_value_into_d(index),
                5,
                PUSH_ONTO_STACK
            )
        }
    }
}

fn write_load_value_into_d(value: &i32) -> String {
    format!("@{}\nD=A", value)
}

fn write_load_value_offset_into_d(segment: &str) -> String {
    format!("@{}\nA=D+M\nD=M", segment)
}

mod tests {
    use super::*;

    #[test]
    fn test_push_arguments() {
        assert_eq!(
            write_code("test", &Command::Push(Segment::Argument, 10)),
            r#"@10
D=A
@ARG
A=D+M
D=M
// push onto stack
@SP
A=M
M=D
@SP
M=M+1"#
        );
    }
}
