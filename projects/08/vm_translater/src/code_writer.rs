use crate::{arithmetic_command::ArithmeticCommand, command::Command, segment::Segment};

pub fn write_bootstrap() -> String {
    vec![
        String::from("// bootstrap"),
        String::from("@256"),
        String::from("D=A"),
        String::from("@SP"),
        String::from("M=D"),
        write_code("bootstrap", &Command::Call("Sys.init".to_string(), 0), &0),
    ]
    .join("\n")
}

pub fn write_code(file_name: &str, command: &Command, id: &i32) -> String {
    match command {
        Command::Arithmetic(arithmetic_command) => write_code_arithmetic(arithmetic_command, id),
        Command::Push(segment, index) => write_code_push(file_name, segment, index),
        Command::Pop(segment, index) => write_code_pop(file_name, segment, index),
        Command::Label(value) => vec![format!("({}${})", file_name, value)].join("\n"),
        Command::GoTo(label) => {
            vec![format!("@{}${}", file_name, label), String::from("0;JMP")].join("\n")
        }
        Command::IfGoTo(label) => vec![
            String::from("@SP"),
            String::from("AM=M-1"),
            String::from("D=M"),
            format!("@{}${}", file_name, label),
            String::from("D;JNE"),
        ]
        .join("\n"),
        Command::Function(function_name, number_of_locals) => {
            let mut out: Vec<String> = Vec::new();
            out.append(&mut vec![
                format!("// function {} {}", function_name, number_of_locals),
                format!("({})", function_name),
            ]);
            for _ in 0i32..*number_of_locals {
                out.append(&mut vec![
                    String::from("@SP"),
                    String::from("A=M"),
                    String::from("M=0"),
                    String::from("@SP"),
                    String::from("M=M+1"),
                ]);
            }
            out.join("\n")
        }
        Command::Call(function_name, number_of_arguments) => {
            vec![
                format!("// call {} {}", function_name, number_of_arguments),
                // push return-address
                format!("@{}_{}", file_name, id),
                String::from("D=A"),
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                String::from("@SP"),
                String::from("M=M+1"),
                // push LCL
                String::from("@LCL"),
                String::from("D=M"),
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                String::from("@SP"),
                String::from("M=M+1"),
                // push ARG
                String::from("@ARG"),
                String::from("D=M"),
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                String::from("@SP"),
                String::from("M=M+1"),
                // push THIS
                String::from("@THIS"),
                String::from("D=M"),
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                String::from("@SP"),
                String::from("M=M+1"),
                // push THAT
                String::from("@THAT"),
                String::from("D=M"),
                String::from("@SP"),
                String::from("A=M"),
                String::from("M=D"),
                String::from("@SP"),
                String::from("M=M+1"),
                // ARG = SP-n-5
                String::from("@SP"),
                String::from("D=M"),
                format!("@{}", number_of_arguments + 5),
                String::from("D=D-A"),
                String::from("@ARG"),
                String::from("M=D"),
                // LCL = SP
                String::from("@SP"),
                String::from("D=M"),
                String::from("@LCL"),
                String::from("M=D"),
                // goto function"
                format!("@{}", function_name),
                String::from("0;JMP"),
                // (return-address)
                format!("({}_{})", file_name, id),
            ]
            .join("\n")
        }
        Command::Return => {
            vec![
                String::from("// return"),
                // FRAME = LCL
                String::from("@LCL"),
                String::from("D=M"),
                String::from("@R13"),
                String::from("M=D"),
                // RET = *(FRAME - 5)
                String::from("@5"),
                String::from("A=D-A"),
                String::from("D=M"),
                String::from("@R14"),
                String::from("M=D"),
                // *ARG = pop()
                String::from("@SP"),
                String::from("AM=M-1"),
                String::from("D=M"),
                String::from("@ARG"),
                String::from("A=M"),
                String::from("M=D"),
                // SP = ARG + 1
                String::from("@ARG"),
                String::from("D=M+1"),
                String::from("@SP"),
                String::from("M=D"),
                // THAT = *(FRAME - 1)
                String::from("@R13"),
                String::from("D=M-1"),
                String::from("AM=D"),
                String::from("D=M"),
                String::from("@THAT"),
                String::from("M=D"),
                // THIS = *(FRAME - 2)
                String::from("@R13"),
                String::from("D=M-1"),
                String::from("AM=D"),
                String::from("D=M"),
                String::from("@THIS"),
                String::from("M=D"),
                // ARG = *(FRAME - 3)
                String::from("@R13"),
                String::from("D=M-1"),
                String::from("AM=D"),
                String::from("D=M"),
                String::from("@ARG"),
                String::from("M=D"),
                // LCL = *(FRAME - 4)
                String::from("@R13"),
                String::from("D=M-1"),
                String::from("AM=D"),
                String::from("D=M"),
                String::from("@LCL"),
                String::from("M=D"),
                // goto RET
                String::from("@R14"),
                String::from("A=M"),
                String::from("0;JMP"),
            ]
            .join("\n")
        }
    }
}

fn write_code_arithmetic(arithmetic_command: &ArithmeticCommand, index: &i32) -> String {
    match arithmetic_command {
        ArithmeticCommand::Add => vec![
            String::from("// add"),
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
            String::from("// sub"),
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
            String::from("// neg"),
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
            String::from("// and"),
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
            String::from("// or"),
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
            String::from("// not"),
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
            format!("// push static {}", index),
            format!("@{}.{}", file_name, index),
            String::from("D=M"),
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
            String::from("// pointer i は 3 + i 番目のアドレスへ 変換されるべき"),
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
            String::from("// temp i は 5 + i 番目のアドレスへ 変換されるべき"),
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
