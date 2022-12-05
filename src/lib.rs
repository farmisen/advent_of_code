use std::{env::current_dir, io::Result, path::PathBuf};

#[cfg(windows)]
pub const LINE_END: &'static str = "\r\n";
#[cfg(windows)]
pub const EMPTY_LINE: &'static str = "\r\n\r\n";
#[cfg(not(windows))]
pub const EMPTY_LINE: &str = "\n\n";
#[cfg(not(windows))]
pub const LINE_END: &str = "\n";

pub fn datapath(puzzle: &str) -> Result<PathBuf> {
    let mut path = current_dir()?;
    path.push("data");
    path.push(puzzle);
    path.push("input.txt");
    Ok(path)
}
