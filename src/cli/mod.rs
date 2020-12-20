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
    Convert(Convert)
}

#[derive(Clap)]
pub struct Convert {
    #[clap(about = "token to translate")]
    token: String,

    #[clap(about = "schemas: ala_lc|bgn_pcgn|bs_2979|gost_52290|gost_7034|icao_doc_9303|gost_779|ungegn_1987|mosmetro|scientific|telegram|wikipedia|yandex_maps|yandex_money")]
    schema: String,
}

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
