use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn assert_absent(path: impl AsRef<Path>) {
    let relative = path.as_ref();
    let path = repo_root().join(relative);
    assert!(
        fs::metadata(&path).is_err(),
        "{} must stay absent; use the accepted DevTape path instead of a parallel substrate",
        relative.display()
    );
}

#[test]
fn forbidden_parallel_substrate_files_are_absent() {
    for path in ["src/cas.rs", "src/hash.rs", "src/versioned_state.rs"] {
        assert_absent(path);
    }
}

