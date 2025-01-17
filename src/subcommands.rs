use clap::Subcommand;

pub mod upgrade;

/// Set of subcommands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Set of subcommands for whiz itself.
    #[clap(subcommand)]
    _Self(SelfCommand),
}

impl Command {
    pub fn run(&self) {
        match self {
            Self::_Self(self_command) => self_command.run(),
        }
    }
}

/// Set of subcommands for whiz itself.
#[derive(Subcommand, Debug)]
pub enum SelfCommand {
    /// Upgrade to the latest version of whiz.
    #[clap(subcommand)]
    Upgrade,
}

impl SelfCommand {
    pub fn run(&self) {
        match self {
            SelfCommand::Upgrade => {
                upgrade::upgrade().expect("whiz should be able to upgrade itself")
            }
        }
    }
}
