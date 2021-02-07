use anyhow::{anyhow, Result};
use code_writer::write_code;
use parser::parse;
use std::io::{BufWriter, Write};
use std::{env, ffi::OsStr};
use std::{fs::File, path::Path};

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
                    let new_file_path = Path::new(dir.path().parent().unwrap())
                        .join(Path::new(dir.path().file_stem().unwrap()).with_extension("asm"));
                    let new_file = File::create(new_file_path)?;
                    let mut writer = BufWriter::new(new_file);
                    for command in commands {
                        writer.write(
                            write_code(dir.file_name().to_str().unwrap(), &command, &id).as_bytes(),
                        )?;
                        writer.write(b"\n\n")?;
                        id += 1;
                    }
                }
            }
        }
    } else {
        let commands = parse(&path_dir.to_path_buf())?;
        let mut id: i32 = 0;
        let new_file_path = Path::new(path_dir.parent().unwrap())
            .join(Path::new(path_dir.file_stem().unwrap()).with_extension("asm"));
        let new_file = File::create(new_file_path)?;
        let mut writer = BufWriter::new(new_file);
        for command in commands {
            writer.write(
                write_code(
                    &path_dir.file_name().unwrap().to_str().unwrap(),
                    &command,
                    &id,
                )
                .as_bytes(),
            )?;
            writer.write(b"\n\n")?;
            id += 1;
        }
    }
    Ok(())
}
