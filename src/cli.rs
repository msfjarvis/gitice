use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};

#[derive(Clap)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
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
#[clap(setting = AppSettings::ColoredHelp)]
pub(crate) struct Freeze {
    /// directory to search and freeze repos from.
    pub(crate) directory: String,
}

/// takes the given
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub(crate) struct Thaw {
    /// directory to put cloned repos into.
    pub(crate) directory: String,
    /// the lockfile to restore repositories from.
    #[clap(default_value = "gitice.lock")]
    pub(crate) lockfile: String,
}
