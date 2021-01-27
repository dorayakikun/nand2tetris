use crate::{arithmetic_command::{self, ArithmeticCommand}, command::Command, segment::Segment};

const PUSH_ONTO_STACK: &str = r#"// Push value onto stack
@SP
A=M
M=D
@SP
M=M+1"#;

const POP_STACK_INTO_D: &str = r#"// Pop stack into d
@SP
AM=M-1
D=M"#;

const POP_STACK_INTO_D_AS_ADDR: &str = r#"// Write d in a general register
@R13
M=D

// Decrese stack pointer and load value into d
@SP
AM=M-1
D=M

// Jump to general register and write d in it
@R13
A=M
M=D"#;

pub fn write_code(file_name: &str, command: &Command) -> String {
    match command {
        Command::Arithmetic(arithmetic_command) => { unimplemented!() },
        Command::Push(segment, index) => write_code_push(file_name, segment, index),
        Command::Pop(segment, index) => write_code_pop(file_name, segment, index),
    }
}

fn write_code_arithmetic(file_name: &str, arithmetic_command: &ArithmeticCommand, index: &i32) -> String {
    match arithmetic_command {
        Add => {
            format!(r#"{}
@SP
A=M-1

M=M+D"#, POP_STACK_INTO_D)
        },
        Sub => {
            format!(r#"{}
@SP
A=M-1

M=M-D"#, POP_STACK_INTO_D)
        },
        Neg => {
            format!(r#"{}
M=-M"#, POP_STACK_INTO_D)
        },
        Eq=> { 
            format!(r#"{}

@SP
A=M-1
D=M-D

@EQ_{}
D;JEQ

@SP
A=M
M=0

@EQ_END_{}
0;JMP

(EQ_{})
@SP
A=M
M=-1

(EQ_END_{})
@SP
M=M+1"#, POP_STACK_INTO_D, index, index, index, index)
         },
        Gt=> { unimplemented!() },
        Lt=> { unimplemented!() },
        And=> { unimplemented!() },
        Or=> { unimplemented!() },
        Not=> { unimplemented!() },
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
                r#"{}

// offset by 3
// pointer i は 3 + i 番目のアドレスへ 変換されるべき
@3
A=D+A
D=M

{}"#,
                write_load_value_into_d(index),
                PUSH_ONTO_STACK
            )
        }
        Segment::Temp => {
            format!(
                r#"{}

// offset by 5
// temp i は 5 + i 番目のアドレスへ 変換されるべき
@5
A=D+A
D=M

{}"#,
                write_load_value_into_d(index),
                PUSH_ONTO_STACK
            )
        }
    }
}

fn write_load_value_into_d(value: &i32) -> String {
    format!(r#"// Load constant or offset into d
@{}
D=A"#, value)
}

fn write_load_value_offset_into_d(segment: &str) -> String {
    format!(r#"// Load value offset and Load offset + address into d
@{}
A=D+M
D=M"#, segment)
}

fn write_code_pop(file_name: &str, segment: &Segment, index: &i32) -> String {
    match segment {
        Segment::Argument => {
            write_pop_stack_into_segment("ARG", index)
        }
        Segment::Local => {
            write_pop_stack_into_segment("LCL", index)
        }
        Segment::Static => {
            format!(r#"{}.{}
D=A

{}"#, file_name, index, POP_STACK_INTO_D_AS_ADDR)
        }
        Segment::Constant => {
            unimplemented!()
        }
        Segment::This => {
            write_pop_stack_into_segment("THIS", index)
        }
        Segment::That => {
            write_pop_stack_into_segment("THAT", index)
        }
        Segment::Pointer => {
            format!(r#"// Load a base address into d
{}
D=A

// base + 3
@3
D=D+A

{}"#, index, POP_STACK_INTO_D_AS_ADDR)
        }
        Segment::Temp => {
            format!(r#"// Load a base address into d
{}
D=A

// base + 5
@5
D=D+A

{}"#, index, POP_STACK_INTO_D_AS_ADDR)
        }
    }
}

fn write_pop_stack_into_segment(segment: &str, index: &i32) -> String {
    format!(r#"// Load offset value into d
@{}
D=A

// Jump to segment + offset
@{}
D=M+D

// Assign address to temp
// R13 - 15 は汎用的なレジスタとしてVM実装で用いることができる
@R13
M=D

// Decrese stack pointer and pop stack into d
@SP
AM=M-1
D=M

// Jump to temp and write d in it
@R13
A=M
M=D"#, index, segment).to_string()
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
