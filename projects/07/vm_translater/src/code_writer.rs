use crate::{command::Command, segment::Segment};

pub fn write_code(file_name: &str, command: &Command) -> String {
    match command {
        Command::Push(segment, index) => match segment {
            Segment::Argument => {
                format!(
                    r#"@{}
D=A
@ARG
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
            },
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
            },
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
                    index,
                    3
                )
            },
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
                    index,
                    5
                )
            },
        },
    }
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
