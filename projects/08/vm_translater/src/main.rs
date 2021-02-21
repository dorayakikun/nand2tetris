use anyhow::{anyhow, Result};
use code_writer::{write_bootstrap, write_code};
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

    if args.len() < 2 {
        return Err(anyhow!("please enter file name"));
    }

    let path_dir = Path::new(&args[1]);

    if path_dir.is_dir() {
        let file_name = path_dir.file_stem().unwrap().to_str().unwrap();
        let new_file_path = path_dir.join(Path::new(file_name).with_extension("asm"));
        let new_file = File::create(new_file_path)?;
        let mut writer = BufWriter::new(new_file);

        let file_names = path_dir.read_dir()?.filter_map(|entry| {
            entry.ok().and_then(|e| e.path().file_name().and_then(|n| n.to_str().map(|s| s.to_string())))
        }).collect::<Vec<String>>();

        if file_names.contains(&"Sys.vm".to_string()) {
            writer.write(write_bootstrap().as_bytes())?;
            writer.write(b"\n\n")?;
        }

        let dirs = path_dir.read_dir()?;
        for dir in dirs {
            let dir = dir?;
            if let Some(extension) = dir.path().extension() {
                if extension == OsStr::new("vm") {
                    let commands = parse(&dir.path())?;
                    let mut id: i32 = 0;
                    for command in commands {
                        writer.write(write_code(dir.path().file_stem().unwrap().to_str().unwrap(), &command, &id).as_bytes())?;
                        writer.write(b"\n\n")?;
                        id += 1;
                    }
                }
            }
        }
    } else {
        if let Some(extension) = path_dir.extension() {
            if extension == OsStr::new("vm") {
                let commands = parse(&path_dir.to_path_buf())?;
                let new_file_path = Path::new(path_dir.parent().unwrap())
                    .join(Path::new(path_dir.file_stem().unwrap()).with_extension("asm"));
                let new_file = File::create(new_file_path)?;
                let mut writer = BufWriter::new(new_file);

                let mut id: i32 = 0;
                for command in commands {
                    writer.write(
                        write_code(
                            &path_dir.file_stem().unwrap().to_str().unwrap(),
                            &command,
                            &id,
                        )
                        .as_bytes(),
                    )?;
                    writer.write(b"\n\n")?;
                    id += 1;
                }
            }
        }
    }
    Ok(())
}
