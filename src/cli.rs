use clap::{AppSettings, Parser};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,
}

#[derive(Parser)]
pub(crate) enum SubCommand {
    Freeze(Freeze),
    Thaw(Thaw),
}

/// recursively find git repos and record their states into a lockfile
#[derive(Parser)]
pub(crate) struct Freeze {
    /// directory to search and freeze repos from.
    pub(crate) directory: String,
}

/// takes the given lockfile and clones them back into the given directory
#[derive(Parser)]
pub(crate) struct Thaw {
    /// directory to put cloned repos into.
    pub(crate) directory: String,
    /// the lockfile to restore repositories from.
    #[clap(default_value = "gitice.lock")]
    pub(crate) lockfile: String,
}
