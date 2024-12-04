use std::path::{Path, PathBuf};

fn common_input(first_dir: impl AsRef<Path>, day: &str, task: &str) -> PathBuf {
    let mut path = parent_of_manifest();
    path.push(first_dir);
    path.push(format!("day{day}"));
    path.push(format!("input{task}.txt"));
    path
}

pub fn input(day: &str, task: &str) -> PathBuf {
    common_input("data", day, task)
}

pub fn test_input(day: &str, task: &str) -> PathBuf {
    common_input("test_data", day, task)
}

fn parent_of_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}
