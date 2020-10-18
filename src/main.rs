use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
struct PersistableRepo {
    pub(crate) path: String,
    pub(crate) remote_url: String,
    pub(crate) head: String,
}

fn main() -> anyhow::Result<()> {
    let dir = match std::env::args().nth(1) {
        Some(d) => d,
        None => {
            println!("Usage:\n  gitice <dir>\n");
            return Ok(());
        }
    };
    let mut repos: Vec<PersistableRepo> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
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
                            upstream
                                .as_str()
                                .unwrap_or("None")
                                .split('/')
                                .collect::<Vec<&str>>()[2],
                        ) {
                            repos.push(PersistableRepo {
                                // Ideally we wanna do this, but it moves `dir`.
                                // path: entry.path().to_string_lossy().strip_prefix(dir).unwrap().to_string(),
                                path: entry.path().to_string_lossy().to_string(),
                                remote_url: remote.url().unwrap_or("None").to_owned(),
                                head: head.to_owned(),
                            });
                        }
                    }
                };
            }
        };
    }
    println!("{:#x?}", repos);
    Ok(())
}
