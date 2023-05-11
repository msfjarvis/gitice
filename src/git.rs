use crate::model::PersistableRepo;
use gix::{sec::trust::DefaultForLevel, Repository, ThreadSafeRepository};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

pub(crate) fn freeze_repos(dir: &str) -> anyhow::Result<()> {
    let mut repos: HashMap<String, PersistableRepo> = HashMap::new();
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        if entry.file_type().is_dir() {
            let path = format!("{}/.git", entry.path().display());
            let git_dir = Path::new(&path);
            if git_dir.exists() && git_dir.is_dir() {
                let mut git_open_opts_map =
                    gix::sec::trust::Mapping::<gix::open::Options>::default();

                // don't use the global git configs
                let config = gix::open::permissions::Config {
                    git_binary: false,
                    system: false,
                    git: false,
                    user: false,
                    env: true,
                    includes: true,
                };
                // change options for config permissions without touching anything else
                git_open_opts_map.reduced =
                    git_open_opts_map
                        .reduced
                        .permissions(gix::open::Permissions {
                            config,
                            ..gix::open::Permissions::default_for_level(gix::sec::Trust::Reduced)
                        });
                git_open_opts_map.full =
                    git_open_opts_map.full.permissions(gix::open::Permissions {
                        config,
                        ..gix::open::Permissions::default_for_level(gix::sec::Trust::Full)
                    });
                let shared_repo =
                    match ThreadSafeRepository::discover_with_environment_overrides_opts(
                        path,
                        Default::default(),
                        git_open_opts_map,
                    ) {
                        Ok(repo) => repo,
                        Err(e) => {
                            return Err(e.into());
                        }
                    };
                let repository = shared_repo.to_thread_local();
                let branch = get_current_branch(&repository);
                let remote = get_remote_for_branch(&repository, branch.as_deref());
                if let Some(branch) = branch &&
                    let Some(remote) = remote &&
                    let Ok(remote_url) = get_url(&repository, &remote) {
                        let relative_path = entry.path().strip_prefix(Path::new(dir))?.to_str().unwrap().to_string();
                        repos.insert(
                            relative_path,
                            PersistableRepo {
                                remote_url,
                                head: branch,
                            },
                        );
                }
            }
        }
    }
    fs::write("gitice.lock", toml::to_string(&repos)?).expect("could not write to lockfile!");
    tracing::info!(
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
            .args([
                "clone",
                &repo.remote_url,
                PathBuf::from(&dir).join(&name).to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run `git clone`. Perhaps git is not installed?");

        if output.status.success() {
            println!("Thawed {name} successfully.");
        } else {
            println!("{}", std::str::from_utf8(&output.stderr)?);
        }
    }

    Ok(())
}

fn get_current_branch(repository: &Repository) -> Option<String> {
    let name = repository.head_name().ok()??;
    let shorthand = name.shorten();

    Some(shorthand.to_string())
}

fn get_remote_for_branch(repository: &Repository, branch_name: Option<&str>) -> Option<String> {
    let branch_name = branch_name?;
    repository
        .branch_remote_name(branch_name)
        .map(|n| n.as_bstr().to_string())
}

fn get_url(repo: &Repository, remote_name: &str) -> anyhow::Result<String> {
    let config = repo.config_snapshot();
    let remotes = match config.plumbing().sections_by_name("remote") {
        Some(sections) => sections,
        None => return Ok(Default::default()),
    };

    let mut remote_url: Option<String> = None;
    for (name, url) in remotes.filter_map(|section| {
        let remote_name = section.header().subsection_name()?;
        let url = section.value("url")?;
        (remote_name, url).into()
    }) {
        remote_url = url.to_string().into();
        if name == remote_name {
            break;
        }
    }

    let remote_url = match remote_url {
        Some(url) => url,
        None => return Ok(Default::default()),
    };

    Ok(remote_url)
}
