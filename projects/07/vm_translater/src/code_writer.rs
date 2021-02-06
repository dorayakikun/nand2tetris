use crate::{arithmetic_command::ArithmeticCommand, command::Command, segment::Segment};

pub fn write_code(file_name: &str, command: &Command, id: &i32) -> String {
    match command {
        Command::Arithmetic(arithmetic_command) => {
            write_code_arithmetic(file_name, arithmetic_command, id)
        }
        Command::Push(segment, index) => write_code_push(file_name, segment, index),
        Command::Pop(segment, index) => write_code_pop(file_name, segment, index),
    }
}

fn write_code_arithmetic(
    file_name: &str,
    arithmetic_command: &ArithmeticCommand,
    index: &i32,
) -> String {
    match arithmetic_command {
        Add => vec![
            String::from("// Add"),
            String::from("// Pop stack into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("@SP"),
            String::from("M=M+D"),
        ]
        .join("\n"),
        Sub => vec![
            String::from("// Sub"),
            String::from("// Pop stack into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("@SP"),
            String::from("M=M-D"),
        ]
        .join("\n"),
        Neg => vec![
            String::from("// Neg"),
            String::from("// Pop stack into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("@SP"),
            String::from("M=-M"),
        ]
        .join("\n"),
        Eq => write_code_comparation("EQ", index),
        Gt => write_code_comparation("GT", index),
        Lt => write_code_comparation("LT", index),
        And => vec![
            String::from("// And"),
            String::from("// Pop stack into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("// M - D"),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("M=D&M"),
        ]
        .join("\n"),
        Or => vec![
            String::from("// Or"),
            String::from("// Pop stack into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("// M - D"),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("M=D|M"),
        ]
        .join("\n"),
        Not => vec![
            String::from("// Not"),
            String::from("// Pop stack into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("// M - D"),
            String::from("@SP"),
            String::from("A=M-1"),
            String::from("M=!M"),
        ]
        .join("\n"),
    }
}

fn write_code_comparation(operator: &str, index: &i32) -> String {
    vec![
        format!("// {}", operator),
        String::from("// Pop stack into d"),
        String::from("@SP"),
        String::from("AM=M-1"),
        String::from("D=M"),
        String::from("// M - D"),
        String::from("@SP"),
        String::from("A=M-1"),
        String::from("D=M-D"),
        format!("// if D > 0 then goto {}_{}", operator, index),
        format!("@{}_{}", operator, index),
        format!("D;J{}", operator),
        String::from("// else set M to false(=0)"),
        String::from("@SP"),
        String::from("A=M"),
        String::from("M=0"),
        format!("// goto {}_END_{}", operator, index),
        format!("@{}_END_{}", operator, index),
        String::from("0;JMP"),
        format!("({}_{})", operator, index),
        String::from("@SP"),
        String::from("A=M"),
        String::from("M=-1"),
        format!("({}_END_{})", operator, index),
        String::from("@SP"),
        String::from("M=M+1"),
    ]
    .join("\n")
}

fn write_code_push(file_name: &str, segment: &Segment, index: &i32) -> String {
    match segment {
        Segment::Argument => write_push_value_into_stack("ARG", index),
        Segment::Local => write_push_value_into_stack("LCL", index),
        Segment::Static => vec![
            String::from("// Static"),
            format!("@{}.{}", file_name, index),
            String::from("D=M"),
            String::from("// Push value onto stack"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        Segment::Constant => vec![
            String::from("// Constant"),
            String::from("// Load constant or offset into d"),
            format!("{}", index),
            String::from("D=A"),
            String::from("// Push value onto stack"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        Segment::This => write_push_value_into_stack("THIS", index),
        Segment::That => write_push_value_into_stack("THAT", index),
        Segment::Pointer => vec![
            String::from("// Pointer"),
            String::from("// Load constant or offset into d"),
            format!("{}", index),
            String::from("D=A"),
            String::from("// offset by 3"),
            String::from("// pointer i は 3 + i 番目のアドレスへ 変換されるべき"),
            String::from("@3"),
            String::from("A=D+A"),
            String::from("D=M"),
            String::from("// Push value onto stack"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        Segment::Temp => vec![
            String::from("// Temp"),
            String::from("// Load constant or offset into d"),
            format!("{}", index),
            String::from("D=A"),
            String::from("// offset by 5"),
            String::from("// temp i は 5 + i 番目のアドレスへ 変換されるべき"),
            String::from("@5"),
            String::from("A=D+A"),
            String::from("D=M"),
            String::from("// Push value onto stack"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
    }
}

fn write_push_value_into_stack(segment: &str, value: &i32) -> String {
    vec![
        format!("// {}", segment),
        String::from("// Load constant or offset into d"),
        format!("@{}", value),
        String::from("D=A"),
        String::from("// Load value offset and Load offset + address into d"),
        format!("@{}", segment),
        String::from("A=D+M"),
        String::from("D=M"),
        String::from("// Push value onto stack"),
        String::from("@SP"),
        String::from("A=M"),
        String::from("M=D"),
        String::from("@SP"),
        String::from("M=M+1"),
    ]
    .join("\n")
}

fn write_code_pop(file_name: &str, segment: &Segment, index: &i32) -> String {
    match segment {
        Segment::Argument => write_pop_stack_into_segment("ARG", index),
        Segment::Local => write_pop_stack_into_segment("LCL", index),
        Segment::Static => vec![
            String::from("// Static"),
            format!("{}.{}", file_name, index),
            String::from("D=A"),
            String::from("// Write d in a general register"),
            String::from("@R13"),
            String::from("M=D"),
            String::from("// Decrese stack pointer and load value into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("// Jump to general register and write d in it"),
            String::from("@R13"),
            String::from("A=M"),
            String::from("M=D"),
        ]
        .join("\n"),
        Segment::Constant => {
            unimplemented!()
        }
        Segment::This => write_pop_stack_into_segment("THIS", index),
        Segment::That => write_pop_stack_into_segment("THAT", index),
        Segment::Pointer => vec![
            String::from("// Pointer"),
            String::from("// Load a base address into d"),
            format!("{}", index),
            String::from("D=A"),
            String::from("// base + 3"),
            String::from("@3"),
            String::from("D=D+A"),
            String::from("// Write d in a general register"),
            String::from("@R13"),
            String::from("M=D"),
            String::from("// Decrese stack pointer and load value into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("// Jump to general register and write d in it"),
            String::from("@R13"),
            String::from("A=M"),
            String::from("M=D"),
        ]
        .join("\n"),
        Segment::Temp => vec![
            String::from("// Temp"),
            String::from("// Load a base address into d"),
            format!("{}", index),
            String::from("D=A"),
            String::from("// base + 5"),
            String::from("@5"),
            String::from("D=D+A"),
            String::from("// Write d in a general register"),
            String::from("@R13"),
            String::from("M=D"),
            String::from("// Decrese stack pointer and load value into d"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("// Jump to general register and write d in it"),
            String::from("@R13"),
            String::from("A=M"),
            String::from("M=D"),
        ]
        .join("\n"),
    }
}

fn write_pop_stack_into_segment(segment: &str, index: &i32) -> String {
    vec![
        format!("// {}", segment),
        String::from("// Load offset value into d"),
        format!("@{}", index),
        String::from("D=A"),
        String::from("// Jump to segment + offset"),
        format!("@{}", segment),
        String::from("D=M+D"),
        String::from("// Assign address to temp"),
        String::from("// R13 - 15 は汎用的なレジスタとしてVM実装で用いることができる"),
        String::from("@R13"),
        String::from("M=D"),
        String::from("// Decrese stack pointer and pop stack into d"),
        String::from("@SP"),
        String::from("AM=M-1"),
        String::from("D=M"),
        String::from("// Jump to temp and write d in it"),
        String::from("@R13"),
        String::from("A=M"),
        String::from("M=D"),
    ]
    .join("\n")
}
