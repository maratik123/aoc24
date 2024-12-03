use std::path::{Path, PathBuf};

fn common_input(first_dir: impl AsRef<Path>, day: &str) -> PathBuf {
    let mut path = parent_of_manifest();
    path.push(first_dir);
    path.push(format!("day{day}"));
    path.push("input.txt");
    path
}

pub fn input(day: &str) -> PathBuf {
    common_input("data", day)
}

pub fn test_input(day: &str) -> PathBuf {
    common_input("test_data", day)
}

fn parent_of_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
