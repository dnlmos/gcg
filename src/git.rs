pub use git2::Repository;
use git2::{DiffFormat, DiffOptions, Error};
use std::path::{Path, PathBuf};

pub fn diff(repo: &Repository, files: &[String]) -> Result<String, git2::Error> {
    let mut ret = String::new();
    let idx = repo.index()?;
    let files: Vec<_> = files.iter().map(|f| Path::new(f)).collect();

    // Get HEAD tree (if any)
    let head_tree = match repo.head() {
        Ok(head) => Some(head.peel_to_tree()?),
        Err(_) => None,
    };

    // 1. Diff HEAD -> Index (staged changes)
    let staged_diff = repo.diff_tree_to_index(head_tree.as_ref(), Some(&idx), None)?;
    staged_diff.print(DiffFormat::Patch, |delta, _, line| {
        if let Some(path) = delta.new_file().path() {
            if files.iter().any(|f| path.ends_with(f)) {
                ret.push(line.origin());
                ret.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
            }
        }
        true
    })?;

    Ok(ret)
}

pub fn get_changed_files(repo: &Repository) -> Result<Vec<PathBuf>, Error> {
    let mut opts = DiffOptions::new();

    // HEAD vs index (staged)
    let head = repo.head()?.peel_to_tree()?;
    let index = repo.index()?;
    let diff = repo.diff_tree_to_index(Some(&head), Some(&index), Some(&mut opts))?;

    let mut changed_files = Vec::new();

    for diff in [&diff] {
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    changed_files.push(path.to_path_buf());
                }
                true
            },
            None,
            None,
            None,
        )?;
    }

    Ok(changed_files)
}
