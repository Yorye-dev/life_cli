use std::fs;
use std::io;
use std::path::Path;

pub fn ensure_dir(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn ensure_file(path: &Path, content: &str) -> io::Result<()> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    Ok(())
}

pub fn read_to_string(path: &Path) -> io::Result<String> {
    if !path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(path)
}

pub fn write_string(path: &Path, content: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = path.with_extension("tmp");
    fs::write(&tmp_path, content)?;
    fs::rename(tmp_path, path)?;
    Ok(())
}

pub fn append_line(path: &Path, line: &str) -> io::Result<()> {
    let mut current = read_to_string(path)?;
    if !current.is_empty() && !current.ends_with('\n') {
        current.push('\n');
    }
    current.push_str(line);
    current.push('\n');
    write_string(path, &current)
}
