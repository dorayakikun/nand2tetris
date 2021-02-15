use crate::{arithmetic_command::ArithmeticCommand, command::Command, segment::Segment};

pub fn write_code(file_name: &str, command: &Command, id: &i32) -> String {
    match command {
        Command::Arithmetic(arithmetic_command) => write_code_arithmetic(arithmetic_command, id),
        Command::Push(segment, index) => write_code_push(file_name, segment, index),
        Command::Pop(segment, index) => write_code_pop(file_name, segment, index),
    }
}

fn write_code_arithmetic(arithmetic_command: &ArithmeticCommand, index: &i32) -> String {
    match arithmetic_command {
        ArithmeticCommand::Add => vec![
            String::from("// Add"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M+D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        ArithmeticCommand::Sub => vec![
            String::from("// Sub"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=M-D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        ArithmeticCommand::Neg => vec![
            String::from("// Neg"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=-M"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        ArithmeticCommand::Eq => write_code_comparation("EQ", index),
        ArithmeticCommand::Gt => write_code_comparation("GT", index),
        ArithmeticCommand::Lt => write_code_comparation("LT", index),
        ArithmeticCommand::And => vec![
            String::from("// And"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=D&M"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        ArithmeticCommand::Or => vec![
            String::from("// Or"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=D|M"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        ArithmeticCommand::Not => vec![
            String::from("// Not"),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("M=!M"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
    }
}

fn write_code_comparation(operator: &str, index: &i32) -> String {
    vec![
        format!("// {}", operator),
        // pop rhs
        String::from("@SP"),
        String::from("AM=M-1"),
        String::from("D=M"),
        // pop lhs
        String::from("@SP"),
        String::from("AM=M-1"),
        String::from("D=M-D"),
        format!("@{}_{}", operator, index),
        format!("D;J{}", operator),
        String::from("@SP"),
        String::from("A=M"),
        String::from("M=0"),
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
            format!("// push constant {}", index),
            format!("@{}", index),
            String::from("D=A"),
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
            format!("// push pointer {}", index),
            // pointer i は 3 + i 番目のアドレスへ 変換されるべき
            format!("@{}", 3 + index),
            String::from("D=M"),
            String::from("@SP"),
            String::from("A=M"),
            String::from("M=D"),
            String::from("@SP"),
            String::from("M=M+1"),
        ]
        .join("\n"),
        Segment::Temp => vec![
            format!("// push temp {}", index),
            // temp i は 5 + i 番目のアドレスへ 変換されるべき
            format!("@{}", 5 + index),
            String::from("D=M"),
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
        format!("// push {} {}", segment, value),
        format!("@{}", value),
        String::from("D=A"),
        format!("@{}", segment),
        String::from("A=D+M"),
        String::from("D=M"),
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
            format!("// pop static {}", index),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{}.{}", file_name, index),
            String::from("M=D"),
        ]
        .join("\n"),
        Segment::Constant => {
            unimplemented!()
        }
        Segment::This => write_pop_stack_into_segment("THIS", index),
        Segment::That => write_pop_stack_into_segment("THAT", index),
        Segment::Pointer => vec![
            format!("// pop pointer {}", index),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{}", 3 + index),
            String::from("M=D"),
        ]
        .join("\n"),
        Segment::Temp => vec![
            format!("// pop temp {}", index),
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{}", 5 + index),
            String::from("M=D"),
        ]
        .join("\n"),
    }
}

fn write_pop_stack_into_segment(segment: &str, index: &i32) -> String {
    vec![
        format!("// pop {} {}", segment, index),
        format!("@{}", index),
        String::from("D=A"),
        format!("@{}", segment),
        String::from("D=M+D"),
        // R13 - 15 は汎用的なレジスタとしてVM実装で用いることができる
        String::from("@R13"),
        String::from("M=D"),
        String::from("@SP"),
        String::from("AM=M-1"),
        String::from("D=M"),
        String::from("@R13"),
        String::from("A=M"),
        String::from("M=D"),
    ]
    .join("\n")
}
