use std::{env, error::Error, path::Path};
mod errors;
fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(errors::JackAnalyzerError::InvalidArgumentLength(args.len()));
    }
    let path_dir = Path::new(&args[1]);
    if path_dir.is_dir() {
    } else {
    }
    Ok(())
}
