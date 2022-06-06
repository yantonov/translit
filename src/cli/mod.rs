use clap::Parser;

#[derive(Parser)]
#[clap(version)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
pub enum Command {
    #[clap(about = "transliterate given token", display_order = 0)]
    Convert(Convert),

    #[clap(about = "schemes commands", display_order = 0)]
    Scheme(Scheme),
}

#[derive(Parser)]
pub struct Convert {
    #[clap(help = "token to translate")]
    token: String,

    #[clap(help = "scheme", short, long)]
    schema: String,
}

#[derive(Parser)]
pub struct Scheme {
    #[clap(subcommand)]
    subcommand: SchemeSubcommand
}

impl Scheme {
    pub fn subcommand(&self) -> &SchemeSubcommand {
        &self.subcommand
    }
}

#[derive(Parser)]
pub enum SchemeSubcommand {
    #[clap(about = "list of available schemas", display_order = 0)]
    List(ListOfAvailableSchemes),
}

#[derive(Parser)]
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
    Arguments { args: Opts::parse() }
}
