use crate::version::VERSION_WITH_COMMIT_HASH;
use clap::Parser;

#[derive(Parser)]
#[clap(version = VERSION_WITH_COMMIT_HASH)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
pub enum Command {
    #[clap(about = "transliterate given token", display_order = 0)]
    Convert(Convert),

    #[clap(about = "schemes commands", display_order = 0)]
    Schemes(Schemes),

    #[clap(
        about = "show the version and the commit hash of this binary",
        display_order = 0
    )]
    Version,
}

#[derive(Parser)]
pub struct Convert {
    #[clap(help = "token to translate")]
    token: String,

    #[clap(help = "scheme", short, long, required_unless_present = "all")]
    schema: Option<String>,

    #[clap(help = "convert with every scheme", long, conflicts_with = "schema")]
    all: bool,
}

#[derive(Parser)]
pub struct Schemes {
    #[clap(subcommand)]
    subcommand: SchemesSubcommand,
}

impl Schemes {
    pub fn subcommand(&self) -> &SchemesSubcommand {
        &self.subcommand
    }
}

#[derive(Parser)]
pub enum SchemesSubcommand {
    #[clap(about = "list of available schemas", display_order = 0)]
    List(ListOfAvailableSchemes),
}

#[derive(Parser)]
pub struct ListOfAvailableSchemes {}

impl Convert {
    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn schema(&self) -> Option<&String> {
        self.schema.as_ref()
    }

    pub fn all(&self) -> bool {
        self.all
    }
}

pub struct Arguments {
    args: Opts,
}

impl Arguments {
    pub fn command(&self) -> &Command {
        &self.args.command
    }
}

pub fn arguments() -> Arguments {
    Arguments {
        args: Opts::parse(),
    }
}
