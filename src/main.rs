#![feature(iter_intersperse)]

use std::{env, path::PathBuf, process};

use anyhow::{Result, anyhow};
use clap::Parser;
use log::debug;
use reqwest::blocking as reqwest;

use crate::{
    git::{Repository, diff, get_changed_files},
    requests::{handle_gemini_request, handle_openai_request},
    schemas::UserMessage,
};

mod git;
mod requests;
mod schemas;

fn main() -> Result<()> {
    env_logger::init();
    if let Err(e) = dotenv::dotenv()
        && !e.not_found()
    {
        return Err(anyhow!(e));
    }
    let args = Args::parse();

    // if path not provided, use current working dir
    let repo_path = args.path.map(Ok).unwrap_or_else(env::current_dir)?;
    let repo = Repository::open(repo_path)?;

    let files = get_changed_files(&repo)?;

    if files.is_empty() {
        eprintln!("Repository has no pending changes.");
        process::exit(2);
    }

    let messages = vec![
        UserMessage {
            role: format!("system"),
            content: format!(
                "You are an AI assistant that generates concise, short and clear Git commit messages from code diffs: \n"
            ),
        },
        UserMessage {
            role: "user".to_string(),
            content: diff(&repo, &files)?,
        },
    ];

    debug!(
        "Sending JSON:\n{}",
        serde_json::to_string_pretty(&messages)?
    );

    let client = reqwest::Client::new();
    if args.provider.gemini {
        handle_gemini_request(&client, &messages)?;
    } else {
        handle_openai_request(&client, &args.provider.openai_api, &messages)?;
    }
    Ok(())
}

/// Generates commit message for current changes in a git repository.
#[derive(Debug, clap::Parser)]
#[command(version)]
struct Args {
    /// Path to git repository
    path: Option<PathBuf>,
    #[command(flatten)]
    provider: Provider,
}

#[derive(Debug, clap::Args)]
#[group(multiple = false)]
struct Provider {
    /// Use Gemini
    #[arg(long)]
    gemini: bool,
    /// Use OpenAI-compatible API
    #[arg(
        long = "openai-api",
        id = "URL",
        default_value = "http://127.0.0.1:11434/v1"
    )]
    openai_api: String,
}
