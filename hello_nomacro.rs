use std::io::Write;

fn main() {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"Hello PH\n").unwrap();
}
