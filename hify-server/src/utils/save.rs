use std::{error::Error, fs, path::Path};

use crate::index::Index;

pub fn save_index(to: &Path, index: &Index) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(index)?;
    fs::write(to, json)?;
    Ok(())
}

pub fn load_index(from: &Path) -> Result<Index, Box<dyn Error>> {
    let content = fs::read(from)?;
    let json_str = std::str::from_utf8(&content)?;
    let json = serde_json::from_str(json_str)?;
    Ok(json)
}
