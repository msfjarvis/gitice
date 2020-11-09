use crate::model::PersistableRepo;
use git2::Repository;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

pub(crate) fn freeze_repos(dir: &str) -> anyhow::Result<()> {
    let mut repos: HashMap<String, PersistableRepo> = HashMap::new();
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
                            upstream.as_str().unwrap().split('/').collect::<Vec<&str>>()[2],
                        ) {
                            let path = entry
                                .path()
                                .strip_prefix(Path::new(dir))?
                                .to_str()
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
    println!(
        "Successfully generated lockfile with {} repos",
        &repos.len()
    );
    Ok(())
}

pub(crate) fn thaw_repos(dir: &str, lockfile: &str) -> anyhow::Result<()> {
    let lockfile = fs::read_to_string(lockfile)
        .unwrap_or_else(|_| panic!("unable to read lockfile from {}", lockfile));
    let repos: HashMap<String, PersistableRepo> = toml::from_str(&lockfile)?;

    for (name, repo) in repos {
        println!("Cloning {} from {}", &name, &repo.remote_url);
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
