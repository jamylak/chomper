use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::process;

const BLOCK_SIZE: usize = 8 * 1024;

fn trim_trailing_newlines(path: &str) -> std::io::Result<u64> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(0);
    }

    let mut end_pos = len;
    let mut buffer = vec![0u8; BLOCK_SIZE];

    while end_pos > 0 {
        let chunk_size = std::cmp::min(BLOCK_SIZE as u64, end_pos) as usize;
        let start_pos = end_pos - chunk_size as u64;
        file.seek(SeekFrom::Start(start_pos))?;
        let slice = &mut buffer[..chunk_size];
        file.read_exact(slice)?;

        if let Some(idx) = slice.iter().rposition(|&b| b != b'\n' && b != b'\r') {
            let new_len = start_pos + idx as u64 + 1;
            if new_len != len {
                file.set_len(new_len)?;
            }
            return Ok(len - new_len);
        }

        end_pos = start_pos;
    }

    file.set_len(0)?;
    Ok(len)
}

fn main() {
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
