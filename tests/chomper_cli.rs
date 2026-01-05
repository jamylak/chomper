use std::fs;
use std::io::Write;
use std::process::Command;

fn write_temp(name: &str, data: &[u8]) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("chomper_{name}_{nanos}.txt"));
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(data).unwrap();
    path
}

fn run_chomper(path: &std::path::Path) {
    let exe = env!("CARGO_BIN_EXE_chomper");
    let status = Command::new(exe)
        .arg(path)
        .status()
        .expect("failed to run chomper");
    assert!(status.success());
}

#[test]
fn trims_multiple_newlines() {
    let path = write_temp("multi", b"hello\n\n\n");
    run_chomper(&path);
    let data = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(data, b"hello");
}

#[test]
fn trims_windows_newlines() {
    let path = write_temp("windows", b"hello\r\n\r\n");
    run_chomper(&path);
    let data = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(data, b"hello");
}

#[test]
fn leaves_file_without_trailing_newline() {
    let path = write_temp("none", b"hello");
    run_chomper(&path);
    let data = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(data, b"hello");
}

#[test]
fn empties_file_with_only_newlines() {
    let path = write_temp("only", b"\n\n");
    run_chomper(&path);
    let data = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(data, b"");
}
