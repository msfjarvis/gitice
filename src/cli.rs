use clap::{crate_version, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = "Harsh Shandilya <me@msfjarvis.dev>")]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,
}

#[derive(Clap)]
pub(crate) enum SubCommand {
    Freeze(Freeze),
    Thaw(Thaw),
}

/// recursively find git repos and record their states into a lockfile
#[derive(Clap)]
pub(crate) struct Freeze {
    /// directory to search and freeze repos from.
    pub(crate) directory: String,
}

/// takes the given
#[derive(Clap)]
pub(crate) struct Thaw {
    /// directory to put cloned repos into.
    pub(crate) directory: String,
    /// the lockfile to restore repositories from.
    #[clap(default_value = "gitice.lock")]
    pub(crate) lockfile: String,
}
