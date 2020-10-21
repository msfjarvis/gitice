use git2::Repository;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct PersistableRepo {
    pub(crate) remote_url: String,
    pub(crate) head: String,
}

fn main() -> anyhow::Result<()> {
    let dir = match std::env::args().nth(2) {
        Some(d) => d,
        None => {
            println!("Usage:\n  gitice <command> <dir>\n");
            return Ok(());
        }
    };

    // temporary solution to support both freezing and thawing
    match std::env::args().nth(1).as_ref().map(|s| &s[..]) {
        Some("freeze") => freeze_repos(dir),
        Some("thaw") => thaw_repos(dir),
        _ => {
            println!("Usage:\n  gitice <command> <dir>\n");
            Ok(())
        }
    }
}

fn freeze_repos(dir: String) -> anyhow::Result<()> {
    let mut repos: HashMap<String, PersistableRepo> = HashMap::new();
    for entry in WalkDir::new(dir.clone()).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            let path = format!("{}/.git", entry.path().display());
            let git_dir = Path::new(&path);

            if git_dir.exists() {
                let repo = Repository::open(git_dir)?;
                if repo.is_empty()? {
                    continue;
                }

                let head = repo.head()?;
                if let Some(head) = head.name() {
                    if let Ok(upstream) = repo.branch_upstream_name(head) {
                        if let Ok(remote) = repo.find_remote(
                            // This is a rather ugly hack, but not sure how else to get the required name
                            // doesn't seem to work with the full name such as `refs/remotes/origin/master`
                            upstream.as_str().unwrap().split('/').collect::<Vec<&str>>()[2],
                        ) {
                            let path = entry
                                .path()
                                .to_string_lossy()
                                .strip_prefix(&dir)
                                .unwrap()
                                .to_string();
                            repos.insert(
                                path,
                                PersistableRepo {
                                    remote_url: remote.url().unwrap_or("None").to_owned(),
                                    head: head.to_owned(),
                                },
                            );
                        }
                    }
                };
            }
        };
    }
    fs::write("gitice.lock", toml::to_string(&repos)?).expect("could not write to lockfile!");
    Ok(())
}

fn thaw_repos(dir: String) -> anyhow::Result<()> {
    let lockfile = fs::read_to_string("gitice.lock").expect("unable to read lockfile!");
    let repos: HashMap<String, PersistableRepo> = toml::from_str(&lockfile)?;

    for (name, repo) in repos {
        let output = Command::new("git")
            .args(&[
                "clone",
                &repo.remote_url,
                PathBuf::from(&dir).join(&name).to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run `git clone`. Perhaps git is not installed?");

        if output.status.success() {
            println!("Thawed {} successfully.", name)
        } else {
            println!("{}", std::str::from_utf8(&output.stderr)?)
        }
    }

    Ok(())
}
