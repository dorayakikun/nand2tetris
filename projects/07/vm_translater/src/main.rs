use anyhow::{anyhow, Result};
use code_writer::write_code;
use parser::parse;
use std::path::Path;
use std::{env, ffi::OsStr};

mod arithmetic_command;
mod code_writer;
mod command;
mod parser;
mod segment;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    if args.len() < 2 {
        return Err(anyhow!("please enter file name"));
    }

    let path_dir = Path::new(&args[1]);

    if path_dir.is_dir() {
        let dirs = path_dir.read_dir()?;
        for dir in dirs {
            let dir = dir?;
            if let Some(extension) = dir.path().extension() {
                if extension == OsStr::new("vm") {
                    let commands = parse(&dir.path())?;
                    let mut id: i32 = 0;
                    for command in commands {
                        println!(
                            "{}",
                            write_code(dir.file_name().to_str().unwrap(), &command, &id)
                        );
                        id += 1;
                    }
                }
            }
        }
    } else {
        let commands = parse(&path_dir.to_path_buf())?;
        let mut id: i32 = 0;
        for command in commands {
            println!(
                "{}",
                write_code(
                    &path_dir.file_name().unwrap().to_str().unwrap(),
                    &command,
                    &id
                )
            );
            id += 1;
        }
    }
    Ok(())
}
