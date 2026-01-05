use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::process;

// Read in fixed-size chunks when scanning from the end of the file.
const BLOCK_SIZE: usize = 8 * 1024;

// Trim trailing newline and carriage return bytes in-place.
fn trim_trailing_newlines(path: &str) -> std::io::Result<u64> {
    // Open for read/write so we can inspect and then truncate if needed.
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(0);
    }

    // Walk backwards to find the last non-newline byte.
    let mut end_pos = len;
    let mut buffer = vec![0u8; BLOCK_SIZE];

    while end_pos > 0 {
        let chunk_size = std::cmp::min(BLOCK_SIZE as u64, end_pos) as usize;
        let start_pos = end_pos - chunk_size as u64;
        file.seek(SeekFrom::Start(start_pos))?;
        let slice = &mut buffer[..chunk_size];
        file.read_exact(slice)?;

        // If this chunk contains content, truncate to just after the last byte.
        if let Some(idx) = slice.iter().rposition(|&b| b != b'\n' && b != b'\r') {
            let new_len = start_pos + idx as u64 + 1;
            if new_len != len {
                file.set_len(new_len)?;
            }
            return Ok(len - new_len);
        }

        // Otherwise keep scanning earlier chunks.
        end_pos = start_pos;
    }

    // File was entirely newlines; truncate to empty.
    file.set_len(0)?;
    Ok(len)
}

fn main() {
    // Parse a single file path argument.
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(p) => p,
        None => {
            eprintln!("usage: chomper <file>");
            process::exit(2);
        }
    };

    if args.next().is_some() {
        eprintln!("usage: chomper <file>");
        process::exit(2);
    }

    // Perform the in-place trim and report failures.
    if let Err(err) = trim_trailing_newlines(&path) {
        eprintln!("chomper: {err}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::trim_trailing_newlines;
    use std::fs;
    use std::io::Write;

    // Write a temporary file with the provided bytes.
    fn write_temp(data: &[u8]) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("chomper_unit_{nanos}.txt"));
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(data).unwrap();
        path
    }

    #[test]
    fn trims_newlines_in_place() {
        let path = write_temp(b"hello\n\n");
        let removed = trim_trailing_newlines(path.to_str().unwrap()).unwrap();
        let data = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        assert_eq!(removed, 2);
        assert_eq!(data, b"hello");
    }
}
