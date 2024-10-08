use std::{fs, path::Path};

use anyhow::Result;

use crate::{index::Index, userdata::UserDataInner};

pub fn save_index(to: &Path, index: &Index) -> Result<()> {
    let json = serde_json::to_string(index)?;
    fs::write(to, json)?;
    Ok(())
}

pub fn load_index(from: &Path) -> Result<Index> {
    let content = fs::read(from)?;
    let json_str = std::str::from_utf8(&content)?;
    let json = serde_json::from_str(json_str)?;
    Ok(json)
}

pub fn save_user_data(to: &Path, user_data: &UserDataInner) -> Result<()> {
    let json = serde_json::to_string(user_data)?;
    fs::write(to, json)?;
    Ok(())
}

pub fn load_user_data(from: &Path) -> Result<UserDataInner> {
    let content = fs::read(from)?;
    let json_str = std::str::from_utf8(&content)?;
    let json = serde_json::from_str(json_str)?;
    Ok(json)
}
