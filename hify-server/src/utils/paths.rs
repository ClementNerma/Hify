use std::path::{Component, Path, PathBuf};

pub fn common_ancestor<P: AsRef<Path>>(paths: impl IntoIterator<Item = P>) -> Option<PathBuf> {
    let mut iter = paths.into_iter();

    // Start with the first path's components
    let first = iter.next()?;
    let mut common: Vec<Component> = first.as_ref().components().collect();

    // Compare with remaining paths
    for path in iter {
        let components: Vec<Component> = path.as_ref().components().collect();

        // Keep only matching prefix components
        common = common
            .into_iter()
            .zip(components)
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();

        if common.is_empty() {
            return None;
        }
    }

    Some(common.iter().collect())
}
