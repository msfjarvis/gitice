use anyhow::anyhow;
use clap::{crate_version, App, AppSettings, Arg};
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
    let matches = App::new("gitice")
        .about("Command-line tool for backing up and restoring multiple Git repositories from a directory")
        .version(crate_version!())
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("freeze")
            .about("Generate a gitice.lock file with all the repositories in the given directory")
            .args(&[Arg::with_name("directory")
                .help("Directory to look for Git repos in")
                .required(true)
                .index(1)]),
            )
        .subcommand(
            App::new("thaw")
                .about("Given a gitice.lock and a directory, clones back all the repositories from the lockfile in the directory")
                .args(&[
                    Arg::with_name("directory")
                    .help("Directory to restore repositories in")
                    .required(true)
                    .index(1),
                    Arg::with_name("lockfile")
                    .help("The lockfile to restore repositories from")
                    .short("l")
                    .long("lockfile")
                    .required(false)
                    .default_value("gitice.lock")
                    ]),
        )
        .get_matches();

    match matches.subcommand() {
        ("freeze", m) => freeze_repos(m.unwrap().value_of("directory").unwrap())?,
        ("thaw", m) => {
            let m = m.unwrap();
            thaw_repos(
                m.value_of("directory").unwrap(),
                m.value_of("lockfile").unwrap(),
            )?
        }
        (cmd, _) => return Err(anyhow!("unknown subcommand: {}", cmd)),
    }

    Ok(())
}

fn freeze_repos(dir: &str) -> anyhow::Result<()> {
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
    println!(
        "Successfully generated lockfile with {} repos",
        &repos.len()
    );
    Ok(())
}

fn thaw_repos(dir: &str, lockfile: &str) -> anyhow::Result<()> {
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
