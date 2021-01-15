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

    #[clap(about = "schemes commands", display_order = 0)]
    Scheme(Scheme),
}

#[derive(Clap)]
pub struct Convert {
    #[clap(about = "token to translate")]
    token: String,

    #[clap(about = "scheme", short, long)]
    schema: String,
}

#[derive(Clap)]
pub struct Scheme {
    #[clap(subcommand)]
    subcommand: SchemeSubcommand
}

impl Scheme {
    pub fn subcommand(&self) -> &SchemeSubcommand {
        &self.subcommand
    }
}

#[derive(Clap)]
pub enum SchemeSubcommand {
    #[clap(about = "list of available schemas", display_order = 0)]
    List(ListOfAvailableSchemes),
}

#[derive(Clap)]
pub struct ListOfAvailableSchemes {}

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
