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
            Segment::This => "".to_string(),
            Segment::That => "".to_string(),
            Segment::Pointer => "".to_string(),
            Segment::Temp => "".to_string(),
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
