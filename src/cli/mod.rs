use clap::{Clap, crate_version};

#[derive(Clap)]
#[clap(version = crate_version ! ())]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
pub enum Command {
    #[clap(about = "transliterate given token", display_order = 0)]
    Convert(Convert),

    #[clap(about = "list of available schemes", display_order = 0)]
    ListSchemes(ListSchemes),
}

#[derive(Clap)]
pub struct Convert {
    #[clap(about = "token to translate")]
    token: String,

    #[clap(about = "scheme")]
    schema: String,
}

#[derive(Clap)]
pub struct ListSchemes {}

impl Convert {
    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn schema(&self) -> &String {
        &self.schema
    }
}

pub struct Arguments {
    args: Opts
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }
}

pub fn arguments() -> Arguments {
    return Arguments { args: Opts::parse() };
}
