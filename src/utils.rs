use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::io::Write;

macro_rules! pathify {
    ($key: expr) => {
        format!("/var/tmp/cache-test/{}", $key)
    };
}

///
/// Write a single key/value pair to disk
///
pub fn write_file(key: &str, value: &str) -> Result<()> {
    let path = pathify!(key);
    let mut file = File::create(path)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

///
/// Read a single file from disk
///
pub fn read_file(key: &str) -> Result<String> {
    let path = pathify!(key);
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

///
/// Remove all files in the `/var/tmp/cache-test` directory
///
pub fn clean_files() -> Result<()> {
    let path = "/var/tmp/cache-test";
    fs::remove_dir_all(path)?;
    Ok(())
}
