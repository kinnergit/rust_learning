use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Lines, Read, Result, Write};
use std::path::PathBuf;

pub fn file_get_contents_with_buffer(path: &str) -> Result<Lines<BufReader<File>>> {
    let path = PathBuf::from(path);

    let fp = File::open(path)?;

    let reader = BufReader::new(fp);

    Ok(reader.lines())
}

pub fn file_get_contents(filename: &str) -> Result<String> {
    let mut buff = String::new();

    let mut f = File::open(filename)?;
    f.read_to_string(&mut buff)?;

    Ok(buff)
}

pub fn file_put_contents(filename: &str, contents: &str) -> Result<usize> {
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)?;
    let size = f.write(contents.as_bytes())?;

    Ok(size)
}
