use crate::{command::Command, segment::Segment};

pub fn write_code(file_name: &str, command: &Command) -> String {
    match command {
        Command::Push(segment, index) => match segment {
            Segment::Argument => {
                format!(
                    "{}\n{}\n\n// push to stack\n{}",
                    write_load_value_into_dregister(index),
                    write_load_value_offset_into_dregister("ARG"),
                    write_push_dregister_onto_stack()
                )
            }
            Segment::Local => {
                format!(
                    r#"@{}
D=A
@LCL
A=D+M
D=M

// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    index
                )
            }
            Segment::Static => {
                format!(
                    r#"@{}.{}
D=M
// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    file_name, index
                )
            }
            Segment::Constant => {
                format!(
                    r#"@{}
D=A
// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    index
                )
            }
            Segment::This => {
                format!(
                    r#"@{}
D=A
@THIS
A=D+M
D=M

// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    index
                )
            }
            Segment::That => {
                format!(
                    r#"@{}
D=A
@THAT
A=D+M
D=M

// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    index
                )
            }
            Segment::Pointer => {
                format!(
                    r#"@{}
D=A

@{}
A=D+A
D=M

// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    index, 3
                )
            }
            Segment::Temp => {
                format!(
                    r#"@{}
D=A

@{}
A=D+A
D=M

// push to stack
@SP
A=M
M=D
@SP
M=M+1"#,
                    index, 5
                )
            }
        },
    }
}

fn write_load_value_into_dregister(value: &i32) -> String {
    format!("@{}\nD=A", value)
}

fn write_load_value_offset_into_dregister(segment: &str) -> String {
    format!("@{}\nA=D+M\nD=M", segment)
}

fn write_push_dregister_onto_stack() -> String {
    r#"@SP
A=M
M=D
@SP
M=M+1"#
        .to_string()
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

// push to stack
@SP
A=M
M=D
@SP
M=M+1"#
        );
    }
}
