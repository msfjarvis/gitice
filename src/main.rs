pub(crate) mod git;
pub(crate) mod model;

use git::freeze_repos;
use git::thaw_repos;

use anyhow::anyhow;
use clap::{crate_version, App, AppSettings, Arg};

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
            .setting(AppSettings::ColoredHelp)
            .args(&[Arg::new("directory")
                .about("Directory to look for Git repos in")
                .required(true)
                .index(1)]),
            )
        .subcommand(
            App::new("thaw")
                .about("Given a gitice.lock and a directory, clones back all the repositories from the lockfile in the directory")
                .setting(AppSettings::ColoredHelp)
                .args(&[
                    Arg::new("directory")
                    .about("Directory to restore repositories in")
                    .required(true)
                    .index(1),
                    Arg::new("lockfile")
                    .about("The lockfile to restore repositories from")
                    .short('l')
                    .long("lockfile")
                    .required(false)
                    .default_value("gitice.lock")
                    ]),
        )
        .get_matches();

    match matches.subcommand().unwrap() {
        ("freeze", m) => freeze_repos(m.value_of("directory").unwrap())?,
        ("thaw", m) => thaw_repos(
            m.value_of("directory").unwrap(),
            m.value_of("lockfile").unwrap(),
        )?,
        (cmd, _) => return Err(anyhow!("unknown subcommand: {}", cmd)),
    }

    Ok(())
}
