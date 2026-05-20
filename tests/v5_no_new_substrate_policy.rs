use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn assert_absent(path: impl AsRef<Path>) {
    let relative = path.as_ref();
    let path = repo_root().join(relative);
    match fs::symlink_metadata(&path) {
        Ok(_) => panic!(
            "{} must stay absent; use the accepted DevTape path instead of a parallel substrate",
            relative.display()
        ),
        Err(err) if err.kind() == ErrorKind::NotFound => {}
        Err(err) => panic!(
            "{} absence check failed with unexpected metadata error: {err}",
            relative.display()
        ),
    }
}

#[test]
fn forbidden_parallel_substrate_files_are_absent() {
    for path in ["src/cas.rs", "src/hash.rs", "src/versioned_state.rs"] {
        assert_absent(path);
    }
}
