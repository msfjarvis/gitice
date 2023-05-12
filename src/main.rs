#![feature(let_chains)]
pub mod cli;
pub mod git;
pub mod logging;
pub mod model;

use clap::Parser;
use cli::{Opts, SubCommand};
use git::{freeze_repos, thaw_repos};

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Freeze(p) => freeze_repos(&p.directory)?,
        SubCommand::Thaw(p) => thaw_repos(&p.directory, &p.lockfile)?,
    }

    Ok(())
}
