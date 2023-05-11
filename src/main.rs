#![feature(let_chains)]
pub(crate) mod cli;
pub(crate) mod git;
pub(crate) mod logging;
pub(crate) mod model;

use clap::Parser;
use cli::{Opts, SubCommand};
use git::freeze_repos;
use git::thaw_repos;

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Freeze(p) => freeze_repos(&p.directory)?,
        SubCommand::Thaw(p) => thaw_repos(&p.directory, &p.lockfile)?,
    }

    Ok(())
}
