use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser)]
pub enum SubCommand {
    Freeze(Freeze),
    Thaw(Thaw),
}

/// recursively find git repos and record their states into a lockfile
#[derive(Parser)]
pub struct Freeze {
    /// directory to search and freeze repos from.
    pub directory: String,
}

/// takes the given lockfile and clones them back into the given directory
#[derive(Parser)]
pub struct Thaw {
    /// directory to put cloned repos into.
    pub directory: String,
    /// the lockfile to restore repositories from.
    #[arg(default_value = "gitice.lock")]
    pub lockfile: String,
}

#[cfg(test)]
mod test {
    use super::Opts;

    #[test]
    fn cli_assert() {
        <Opts as clap::CommandFactory>::command().debug_assert();
    }
}
