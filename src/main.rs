use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    eprintln!("Running for: {}", args[1]);
    remove_trailing_empty_lines(path)
}

fn remove_trailing_empty_lines<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let file = OpenOptions::new().read(true).open(&path)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    while let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
        } else {
            break;
        }
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(&path)?;
    for line in &lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
