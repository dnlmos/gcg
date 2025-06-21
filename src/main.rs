use std::error::Error;

use crate::git::{diff, get_changed_files, open_repo};

mod git;

fn main() -> Result<(), Box<dyn Error>> {
    let repo_path = String::from("../cognito");
    let repo = open_repo(&repo_path)?;

    let mut files: Vec<String> = Vec::new();

    match get_changed_files(&repo) {
        Ok(changed_files) => {
            for file in changed_files {
                files.push(file.display().to_string());
                println!("{}", file.display().to_string());
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match diff(&repo, &files) {
        Ok(diff_output) => {
            if diff_output.is_empty() {
                println!("No matching diffs.");
            } else {
                println!("{}", diff_output);
            }
        }
        Err(e) => eprintln!("Error generating diff: {}", e),
    }

    Ok(())
}
